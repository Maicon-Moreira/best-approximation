use core::f64::consts::{
    E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4,
    FRAC_PI_6, FRAC_PI_8, LN_10, LN_2, LOG10_2, LOG10_E, LOG2_10, LOG2_E, PI, SQRT_2, TAU,
};

pub const PHI: f64 = 1.618_033_988_749_895_f64;
pub const FRAC_1_SQRT_PI: f64 = 0.564_189_583_547_756_3_f64;
pub const SQRT_3: f64 = 1.732_050_807_568_877_2_f64;
pub const FRAC_1_SQRT_3: f64 = 0.577_350_269_189_625_7_f64;

pub const MAX_DEPTH: usize = 8;
pub const TOTAL_FORMULAS: usize = 8;

pub const CONSTANTS_WITH_NAMES: [(f64, &str); 23] = [
    (PI, "Pi"),
    (TAU, "Tau"),
    (PHI, "Phi"),
    (E, "E"),
    (SQRT_2, "Sqrt(2)"),
    (SQRT_3, "Sqrt(3)"),
    (LOG2_10, "Log2(10)"),
    (LOG2_E, "Log2(E)"),
    (LOG10_2, "Log10(2)"),
    (LOG10_E, "Log10(E)"),
    (LN_2, "Ln(2)"),
    (LN_10, "Ln(10)"),
    (FRAC_PI_2, "Pi/2"),
    (FRAC_PI_3, "Pi/3"),
    (FRAC_PI_4, "Pi/4"),
    (FRAC_PI_6, "Pi/6"),
    (FRAC_PI_8, "Pi/8"),
    (FRAC_1_PI, "1/Pi"),
    (FRAC_1_SQRT_PI, "1/Sqrt(Pi)"),
    (FRAC_2_PI, "2/Pi"),
    (FRAC_2_SQRT_PI, "2/Sqrt(Pi)"),
    (FRAC_1_SQRT_2, "1/Sqrt(2)"),
    (FRAC_1_SQRT_3, "1/Sqrt(3)"),
];
