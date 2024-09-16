use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use std::collections::HashSet;

use crate::expression::Expression;

#[derive(Clone, Copy, Debug)]
struct HashableF64(f64);

impl PartialEq for HashableF64 {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl Eq for HashableF64 {}

impl Hash for HashableF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[derive(Debug)]
pub struct BestExpressions {
    expressions: Vec<Expression>,
    highest_error: f64,
    target_size: usize,
    unique_error_values: HashSet<HashableF64>,
}

impl BestExpressions {
    pub fn new(target_size: usize) -> Self {
        return Self {
            expressions: Vec::with_capacity(target_size * 2),
            highest_error: f64::MAX,
            target_size,
            unique_error_values: HashSet::new(),
        };
    }

    pub fn add(&mut self, expression: &Expression) {
        if expression.get_last_error() < self.highest_error {
            if expression.is_invalid() {
                return;
            }
            let error = HashableF64(expression.get_last_error());
            if self.unique_error_values.contains(&error) {
                return;
            }
            self.unique_error_values.insert(error);

            self.expressions.push(expression.clone());
            if self.expressions.len() >= self.target_size * 2 {
                self.crop();
            }
        }
    }

    fn crop(&mut self) {
        self.expressions.sort_by(|a, b| {
            return a
                .get_last_error()
                .partial_cmp(&b.get_last_error())
                .unwrap_or(Ordering::Equal);
        });
        self.expressions.truncate(self.target_size);
        self.highest_error = self
            .expressions
            .last()
            .map_or(f64::MAX, |expr| return expr.get_last_error());
    }

    pub fn finalize(&mut self) {
        self.crop();
        self.expressions.sort_by(|a, b| {
            return a
                .get_last_error()
                .partial_cmp(&b.get_last_error())
                .unwrap_or(Ordering::Equal);
        });
    }

    pub fn get_expressions(&self) -> &[Expression] {
        return &self.expressions;
    }

    pub fn first_expression_len(&self) -> usize {
        return self.expressions.first().expect("No expressions").len();
    }

    pub fn has_any_expression(&self) -> bool {
        return !self.expressions.is_empty();
    }
}
