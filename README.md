# mealplan-rs

## Description

My partner and I hate deciding weekly meals, so why not let RNG decide
for us?

## Basic Functionality

* Have a `Meal` data structure which stores information about ingredients,
difficulty, number of times cooked, etc.
* Read in `Meal` information from a JSON file.
* Read in previous weekly meals from a JSON file.
* Pseudo-randomly select seven meals with heuristic favoring less cooked meals.
* Prints out ingredient list for selected meals (allowing repeats, instead of
aggregates).

## Stretch Goals

* Aggregate overlapping ingredients.
* Reads in ingredients list and generates recipes maximizes ingredient overlap.
* Allows for user input on meals they do or do not want (including genres) using
y/n prompts.
* Extend to storing/reading beyond locally hosted JSON files:
    * Pulls in meal information from Google sheets for improved accessibility.
    * Track meal history and store in Gogole sheets / some DB.
* Generate visuals (idk what for... but they will be visual)
* Searchs web for new recipes and writes them to the JSON file of recipes.
