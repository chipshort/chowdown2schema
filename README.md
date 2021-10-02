# chowdown2schema

Reads [chowdown](https://github.com/clarklab/chowdown) recipes and converts them to [schema.org recipe format](https://schema.org/Recipe) (e.g. to import into [Nextcloud Cookbook](https://apps.nextcloud.com/apps/cookbook)).

This is essentially just a really quick and dirty script to ease importing my chowdown recipes into Nextcloud.
Because of this, there is no error handling and the usability is very questionable.

## Usage

Run the binary inside your chowdown folder or copy the `_recipes` folder of your chowdown repository into this one.
The resulting json files are created inside the `_recipes` folder.