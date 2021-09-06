
use std::ops;

static DENOTATION_HALF_SCALE: i64 = 1000000000;
static DENOTATION_SCALE: i64 = DENOTATION_HALF_SCALE * DENOTATION_HALF_SCALE;

#[derive(Copy, Clone)]
pub struct Denotation
{
    val: i128
}

impl Denotation
{
    pub fn new_i128(v: i128) -> Denotation
    {
        Denotation {val: v * DENOTATION_SCALE as i128}
    }
    
    pub fn new_f64(v: f64) -> Denotation
    {
        Denotation {val: (v * DENOTATION_SCALE as f64) as i128}
    }

    pub fn new_str(v: &str) -> Denotation
    {
        let mut val: i128 = 0;
        let mut sign: bool = false;
        let mut decimal_pos: i8 = 0;
        let mut decimal_found: bool = false;
        for c in v.chars() {
            if c >= '0' && c <= '9' {
                val = val*10 + (c as i8 -'0' as i8) as i128;
                if decimal_found {
                    decimal_pos += 1;
                }
            }
            else if c == '.' {
                decimal_found = true;
            }
            else if c == '-' {
                sign = !sign;
            }
        }
        val = if sign {-val} else {val};
        for _i in decimal_pos..18 {
            val *= 10;
        }
        Denotation {val}
    }
    
    pub fn get_i64(self) -> i64
    {
        (self.val / DENOTATION_SCALE as i128) as i64
    }

    pub fn get_f64(self) -> f64
    {
        self.val as f64 / DENOTATION_SCALE as f64
    }
}

impl ops::AddAssign<Denotation> for Denotation
{
    fn add_assign(&mut self, other: Self)
    {
        *self = Self {val: self.val + other.val};
    }
}

impl ops::Add<Denotation> for Denotation
{
    type Output = Denotation;

    fn add(self, other: Denotation) -> Denotation
    {
        let mut out = Denotation{val: self.val};
        out += other;
        out
    }
}

#[cfg(test)]
mod test {
    use super::Denotation;

    #[test]
    fn denotation_new_i128_1() {
        assert_eq!(Denotation::new_i128(1).get_i64(), 1)
    }
    #[test]
    fn denotation_new_f64_1() {
        assert_eq!(Denotation::new_f64(1.).get_i64(), 1)
    }
    #[test]
    fn denotation_new_str_1() {
        assert_eq!(Denotation::new_str("1").get_i64(), 1)
    }
    #[test]
    fn denotation_new_f64_m1() {
        assert_eq!(Denotation::new_f64(-1.).get_i64(), -1)
    }
    #[test]
    fn denotation_new_i128_m1() {
        assert_eq!(Denotation::new_i128(-1).get_i64(), -1)
    }
    #[test]
    fn denotation_new_str_m1() {
        assert_eq!(Denotation::new_str("-1").get_i64(), -1)
    }
    #[test]
    fn denotation_new_f64_2() {
        assert_eq!(Denotation::new_f64(2.).get_f64(), 2.)
    }
    #[test]
    fn denotation_new_f64_1p5() {
        assert_eq!(Denotation::new_f64(1.5).get_f64(), 1.5)
    }
    #[test]
    fn denotation_new_f64_m1p5() {
        assert_eq!(Denotation::new_f64(-1.5).get_f64(), -1.5)
    }
    #[test]
    fn denotation_new_f64_m1p5_to_i64() {
        assert_eq!(Denotation::new_f64(-1.5).get_i64(), -1)
    }
    #[test]
    fn denotation_new_f64_p5_to_i64() {
        assert_eq!(Denotation::new_f64(0.5).get_i64(), 0)
    }
    #[test]
    fn denotation_new_f64_p5() {
        assert_eq!(Denotation::new_f64(0.5).get_f64(), 0.5)
    }
    #[test]
    fn denotation_new_str_m1p5() {
        assert_eq!(Denotation::new_str("-1.5").get_f64(), -1.5)
    }
    #[test]
    fn denotation_new_str_m123456p1250() {
        assert_eq!(Denotation::new_str("-123456.1250").get_f64(), -123456.125)
    }
    #[test]
    fn denotation_new_f64_p00000025() {
        assert_eq!(Denotation::new_f64(0.00000025).get_f64(), 0.00000025)
    }
    #[test]
    fn denotation_new_i128_9999() {
        assert_eq!(Denotation::new_i128(9999).get_i64(), 9999)
    }

}



