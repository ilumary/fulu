use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use std::fs::OpenOptions;

#[derive(Deserialize, Serialize)]
pub struct RecipeCollection {
    collection_name: String,
    recipes: Vec<Recipe>,
}

#[derive(Deserialize, Serialize)]
pub struct Recipe {
    recipe_name: String,
    cooking_time: u32,
    description: String,
    ingredients: std::collections::HashMap<String, String>,
}

impl RecipeCollection {
    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    pub fn get_specific_recipe_of_collection(collection: RecipeCollection, name: &str) -> Option<Recipe> {
        for r in collection.recipes {
            if r.recipe_name.eq(name) {
                return Some(r);
            }
        }
        None
    }

    pub fn delete_recipe(&mut self, recipe: Recipe) {
        self.recipes.retain(|x| *x.description != recipe.description 
                                && *x.recipe_name != recipe.recipe_name 
                                && x.cooking_time != recipe.cooking_time 
                                && x.ingredients != recipe.ingredients
        );
    }

    pub fn create_new_collection(name: String) -> Self {
        let rc = RecipeCollection {
            collection_name: name,
            recipes: Vec::new(),
        };
        rc
    }

    pub fn read_collection_from_file(file: &str) -> Self {
        let value: RecipeCollection = toml::from_str(std::fs::read_to_string(file).expect("Failed to read file").as_ref()).unwrap();
        value
    }

    pub fn save_to_file(&self, file: &str) -> std::io::Result<()> {
        if std::path::Path::new(file).exists() {
            std::fs::remove_file(file).unwrap();
        }
        std::fs::File::create(file).unwrap();
    
        let mut file = OpenOptions::new().write(true).append(false).open(file).unwrap();
    
        let toml = toml::to_string(self).unwrap();
        file.write(&toml.as_bytes())?;
        Ok(())
    }
}

impl Recipe {

    // getters aka immutable access
    pub fn name(&self) -> &String { &self.recipe_name }
    pub fn minutes(&self) -> &u32 { &self.cooking_time }
    pub fn description(&self) -> &String { &self.description }
    pub fn ingredients(&self) -> &std::collections::HashMap<String, String> { &self.ingredients }

    // setters aka mutable access
    pub fn name_mut(&mut self) -> &mut String { &mut self.recipe_name }
    pub fn minutes_mut(&mut self) -> &mut u32 { &mut self.cooking_time }
    pub fn description_mut(&mut self) -> &mut String { &mut self.description }

    pub fn recipe_builder(name: String, pminutes: u32, pdescription: String, pingredients: std::collections::HashMap<String, String>) -> Self {
        let rec = Recipe {
            recipe_name: name,
            cooking_time: pminutes,
            description: pdescription,
            ingredients: pingredients,
        };
        rec
    }

    pub fn add_ingredient(&mut self, ingredient_name: String, ingredient_amount: String) {
        self.ingredients.insert(ingredient_name, ingredient_amount);
    }

    pub fn remove_ingredient(&mut self, ingredient_name: String) -> Result<(), &'static str>{
        match self.ingredients.remove(&ingredient_name) {
            Some(_) => Ok(()),
            None => Err("ingredient is not in list"),
        }
    }

    pub fn does_ingredient_exist(&self, ingredient_name: String) -> bool {
        self.ingredients.contains_key(&ingredient_name)
    }
}