use crate::{best_expressions::BestExpressions, expression::Expression};
use core::fmt::Write;

pub struct ExpressionGenerator {
    max_depth: usize,
    target_constant: f64,
    constant_name: String,
    best_expressions_at_depth: Vec<BestExpressions>,
}

impl ExpressionGenerator {
    pub fn new(
        max_depth: usize,
        target_constant: f64,
        constant_name: String,
        num_formulas: usize,
    ) -> Self {
        return Self {
            max_depth,
            target_constant,
            constant_name,
            best_expressions_at_depth: (0..=max_depth)
                .map(|_| return BestExpressions::new(num_formulas))
                .collect(),
        };
    }

    pub fn generate(&mut self) {
        let mut empty_expression = Expression::new(self.target_constant);
        self.recursive_search(&mut empty_expression, self.max_depth);
        self.best_expressions_at_depth
            .iter_mut()
            .for_each(|best_expressions| return best_expressions.finalize());
    }

    fn recursive_search(&mut self, expression: &mut Expression, depth: usize) {
        self.best_expressions_at_depth
            .get_mut(depth)
            .expect("Invalid depth")
            .add(expression);
        if depth > 0 {
            let possible_next_chars = expression.get_possible_next_chars();
            for c in possible_next_chars {
                expression.push(c);
                self.recursive_search(expression, depth - 1);
                expression.pop();
            }
        }
    }

    pub fn to_markdown_string(&self) -> String {
        let mut readme_string = String::new();

        write!(
            &mut readme_string,
            "# {}\n\nTarget name: **{}**\n\nTarget value: **{:.15}**\n\n",
            self.constant_name, self.constant_name, self.target_constant
        )
        .expect("Unable to write to string");

        for best_expressions in self.best_expressions_at_depth.iter().rev() {
            if !best_expressions.has_any_expression() {
                continue;
            }
            let expression_size = best_expressions.first_expression_len();
            write!(
                &mut readme_string,
                "With `{expression_size}` characters:\n| Formula | Value | Error |\n| --- | --- | --- |\n",
            )
            .expect("Unable to write to string");
            for expression in best_expressions.get_expressions() {
                writeln!(
                    &mut readme_string,
                    "| `{}` | {:.6} | {:.6} |",
                    expression.get(),
                    expression.get_last_evaluation(),
                    expression.get_last_error()
                )
                .expect("Unable to write to string");
            }
            readme_string.push_str("---\n\n");
        }

        return readme_string;
    }
}
