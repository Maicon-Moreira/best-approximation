use crate::char::Char;

#[derive(Debug, Clone)]
pub struct Expression {
    string: String,
    chars: Vec<Char>,
    target_constant: f64,
    evaluation: Vec<f64>,
    error: Vec<f64>,
}

impl Expression {
    pub fn new(target_constant: f64) -> Self {
        return Self {
            string: String::new(),
            chars: vec![Char::new(None)],
            target_constant,
            evaluation: vec![f64::MAX],
            error: vec![f64::MAX],
        };
    }

    pub fn get_last_evaluation(&self) -> f64 {
        return *self.evaluation.last().expect("No evaluations");
    }

    pub fn get_last_error(&self) -> f64 {
        return *self.error.last().expect("No errors");
    }

    pub fn get_last_char(&self) -> &Char {
        return self.chars.last().expect("No chars");
    }

    pub fn get_possible_next_chars(&self) -> Vec<char> {
        return self.get_last_char().get_possible_next_chars();
    }

    pub fn evaluate(&mut self) {
        let mut namespace = fasteval::EmptyNamespace;

        let current_eval = fasteval::ez_eval(&self.string, &mut namespace).unwrap_or(f64::MAX);
        self.evaluation.push(current_eval);
        self.error.push((self.target_constant - current_eval).abs());
    }

    pub fn push(&mut self, c: char) {
        self.chars.push(Char::new(Some(c)));
        self.string.push(c);

        self.evaluate();
    }

    pub fn pop(&mut self) {
        self.chars.pop();
        self.string.pop();
        self.evaluation.pop();
        self.error.pop();
    }

    pub fn is_simple_float(&self) -> bool {
        return self
            .string
            .chars()
            .all(|c| return c.is_ascii_digit() || c == '.' || c == '-');
    }

    pub fn ends_with_digit(&self) -> bool {
        return self
            .string
            .chars()
            .last()
            .map_or(false, |c| return c.is_ascii_digit());
    }

    pub fn is_invalid(&self) -> bool {
        return !self.ends_with_digit() || self.is_simple_float();
    }

    pub fn get(&self) -> String {
        return self.string.clone();
    }

    pub fn len(&self) -> usize {
        return self.string.len();
    }
}
