
fn main() {
    let mut collection = recipeapi::RecipeCollection::create_new_collection("Various Vegan Recipes".to_string());
    let mut recipe1 = recipeapi::Recipe::recipe_builder("Waffles".to_string(), 30, "Very Tasty Waffles".to_string(), std::collections::HashMap::new());
    recipe1.add_ingredient("Eggs".to_string(), "2".to_string());
    recipe1.add_ingredient("Flour".to_string(), "200g".to_string());
    recipe1.add_ingredient("Milk".to_string(), "300ml".to_string());
    let mut recipe2 = recipeapi::Recipe::recipe_builder("Gin Tonic".to_string(), 30, "Strong Alcohol".to_string(), std::collections::HashMap::new());
    recipe2.add_ingredient("Gin".to_string(), "2cl".to_string());
    recipe2.add_ingredient("Tonic".to_string(), "2cl".to_string());
    recipe2.add_ingredient("Grapefruit".to_string(), "2 sclices".to_string());

    collection.add_recipe(recipe1);
    collection.add_recipe(recipe2);

    collection.save_to_file("recipe_collection.toml").unwrap();
}
