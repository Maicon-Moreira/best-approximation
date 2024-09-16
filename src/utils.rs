use std::{collections::HashMap, fs::write, sync::Mutex, time::Instant};

use crate::{
    constants::{CONSTANTS_WITH_NAMES, MAX_DEPTH, TOTAL_FORMULAS},
    expression_generator::ExpressionGenerator,
};
use core::fmt::Write;
use rayon::prelude::*;

pub fn string_to_markdown_link(s: &str) -> String {
    return s
        .to_lowercase()
        .chars()
        .filter(|c| return c.is_alphanumeric())
        .collect();
}

pub fn generate_readme() {
    let markdown_strings_lock = Mutex::new(HashMap::new());

    CONSTANTS_WITH_NAMES
        .par_iter()
        .for_each(|&(constant, name)| {
            let mut generator =
                ExpressionGenerator::new(MAX_DEPTH, constant, String::from(name), TOTAL_FORMULAS);
            let init_time = Instant::now();
            generator.generate();
            let elapsed = init_time.elapsed();

            println!("Elapsed time: {elapsed:.2?} for {name}");

            let mut markdown_strings = markdown_strings_lock.lock().expect("Unable to lock");
            markdown_strings.insert(name, generator.to_markdown_string());
        });

    let markdown_strings = markdown_strings_lock
        .into_inner()
        .expect("Unable to get inner");

    let mut final_string = String::new();

    write!(&mut final_string, "# Best approximation\n\nAlternative ways to approximate mathematical constants.\n\n[Go to index of constants](#index)\n\n").expect("Unable to write to string");

    for &(_, constant_name) in &CONSTANTS_WITH_NAMES {
        write!(
            &mut final_string,
            "{}\n\n",
            markdown_strings
                .get(constant_name)
                .expect("No markdown string")
        )
        .expect("Unable to write to string");
    }

    write!(&mut final_string, "# Index\n\n").expect("Unable to write to string");
    for &(_, constant_name) in &CONSTANTS_WITH_NAMES {
        write!(
            &mut final_string,
            "[Go to `{constant_name}` formulas](#{})\n\n",
            string_to_markdown_link(constant_name)
        )
        .expect("Unable to write to string");
    }

    write("README.md", final_string).expect("Unable to write to file");
}
