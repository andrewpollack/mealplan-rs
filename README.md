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

## Learnings
* [This bullet refers to this commit](https://github.com/andrewpollack/mealplan-rs/commit/defdc0e2657e377bb39977a1941599711b78717a#diff-eb0f73a8f5904cc4492d0a5b224467477efd62007011c7bb06c9e015a58229e6R26).
While creating the `Meal` struct, I thought it would make sense to create an
accompanying `Meal::new` functionality. I tried to use a `HashMap`, but started
running into issues around varying value types to map against the keys. We have
`String`, `u8`, and `Vec<String>`. While I could use some functionality around
`Box` or related enums, I felt like it was beginning to overcomplicate the implementation.
I settled on having a "metadata" to capture all non-Vec information, but on review,
I may just rework `Meal::new` to capture its individual members as-is, or do away
with it entirely. This decision will come down to whether `Meal` will have any
functions outside of getters.

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
