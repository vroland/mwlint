#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate mwlint;
extern crate mediawiki_parser;
extern crate serde_json;

use wasm_bindgen::prelude::*;

/// Naive linter function. Outputs result as serialized JSON.
#[wasm_bindgen]
pub fn lint(input: &str) -> String {
    let settings = mwlint::Settings::default();

    let mut tree = match mediawiki_parser::parse(&input) {
        Ok(elem) => elem,
        Err(mwerror) => return serde_json::to_string(&mwerror)
            .expect("could not serialize"),
    };

    tree = match mwlint::normalize(tree, &settings) {
        Ok(elem) => elem,
        Err(mwerror) => return serde_json::to_string(&mwerror)
            .expect("could not serialize"),
    };

    let mut rules = mwlint::get_rules();
    let mut lints = vec![];

    for mut rule in &mut rules {
        rule.run(&tree, &settings, &mut vec![])
            .expect("error while checking rule!");
        lints.append(&mut rule.lints().iter().map(|l| l.clone()).collect())
    }

    serde_json::to_string(&lints)
        .expect("could not serialize lints")
}

/// Lint examples as JSON string.
#[wasm_bindgen]
pub fn examples() -> String {
    let rules = mwlint::get_rules();
    let examples = rules.iter().fold(vec![], |mut vec, rule| {
        vec.append(&mut rule.examples().clone());
        vec
    });

    serde_json::to_string(&examples)
        .expect("could not serialize examples")
}

