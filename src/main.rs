use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetFormat {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub prep_time: String,
    pub cook_time: String,
    pub total_time: String,
    pub recipe_category: String,
    pub keywords: String,
    pub recipe_yield: i64,
    pub tool: Vec<::serde_json::Value>,
    pub recipe_ingredient: Vec<String>,
    pub recipe_instructions: Vec<String>,
    pub nutrition: Vec<::serde_json::Value>,
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    pub date_modified: String,
    pub date_created: String,
}

fn main() {
    println!("Hello, world!");

    for entry in std::fs::read_dir("./_recipes").unwrap() {
        let e = entry.unwrap();
        let mut path = e.path();
        if path.is_dir() {
            continue;
        }
        let recipe = parse_file(&path.to_str().unwrap());
        let json = serde_json::to_string(&recipe).unwrap();
        path.set_extension("");
        path.set_file_name(recipe.name);
        path.push("recipe.json");
        // println!("{:?}", path.parent().to_owned().unwrap());
        let parent = path.parent().unwrap();
        let mut nomedia = parent.to_owned();
        std::fs::create_dir(parent).unwrap_or(());
        std::fs::write(path, json).unwrap();
        nomedia.push(".nomedia");
        std::fs::write(nomedia, &[]).unwrap();
        // println!("{:?}", path);
    }
}

fn parse_file(path: &str) -> TargetFormat {
    const SKIP_LEN: usize = "---".as_bytes().len();

    let file = std::fs::read_to_string(path).unwrap();
    // let file = std::fs::read_to_string("./_recipes/bauerntopf.md").unwrap();
    let start = file.find("---").unwrap() + SKIP_LEN;
    let remaining = &file[start..];
    let end = remaining.find("---").unwrap();
    let yaml_str = &remaining[0..end];
    let comment = remaining[(end + SKIP_LEN)..].trim();

    // println!("{}", yaml_str);
    // println!("{}", comment);

    let yamls = yaml_rust::YamlLoader::load_from_str(yaml_str).unwrap();
    let y = &yamls[0];
    let title = y["title"].as_str().unwrap();
    let tags = y["tags"].as_str().unwrap().replace(' ', ",");
    let ingredients: Vec<_> = y["ingredients"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|i| i.as_str().unwrap().to_string())
        .collect();
    let directions: Vec<_> = y["directions"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|d| d.as_str().unwrap().to_string())
        .collect();

    // println!("{:#?}", y);

    TargetFormat {
        id: 0,
        name: title.to_string(),
        description: comment.to_string(),
        url: "".to_string(),
        image: "".to_string(),
        prep_time: "".to_string(),
        cook_time: "".to_string(),
        total_time: "".to_string(),
        recipe_category: "".to_string(),
        keywords: tags,
        recipe_yield: 1,
        tool: vec![],
        recipe_ingredient: ingredients,
        recipe_instructions: directions,
        nutrition: vec![],
        context: "http://schema.org".to_string(),
        type_field: "Recipe".to_string(),
        date_modified: "2021-09-21T16:56:21+0000".to_string(),
        date_created: "2021-09-21T16:56:21+0000".to_string(),
    }
}
