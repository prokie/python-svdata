use std::fmt;
use std::ops::{Add, Mul, Neg, Shl, Shr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvPrimaryLiteralIntegral {
    pub data_01: Vec<usize>,
    pub data_xz: Option<Vec<usize>>,
    pub size: usize,
    pub signed: bool,
}

/// The following functions should be replaced by the build in methods once they become stable.
/// All the test cases were created with usize::BITS = 64 although all the methods support any usize::BITS
impl SvPrimaryLiteralIntegral {
    /** Unsigned addition between two integral primary literals.
    Both data_01 vector dimensions (i.e nu of elements) are matched.
    It can be used for "signed" and "unsigned" values, and therefore the final number of bits is not derived within the function.
    Instead it must be explicitly implemented according the context that the function is used. */
    pub fn _unsigned_primlit_add(&mut self, mut right_nu: SvPrimaryLiteralIntegral) {
        self._primlit_vec_elmnt_match(&mut right_nu);

        let mut carry_flag: bool = false;

        for x in 0..self.data_01.len() {
            let left_nu: usize = self.data_01[x];
            self.data_01[x] = left_nu.wrapping_add(right_nu.data_01[x]);

            if carry_flag {
                self.data_01[x] = self.data_01[x].wrapping_add(1);
            }

            if self.data_01[x] >= left_nu && self.data_01[x] >= right_nu.data_01[x] {
                carry_flag = false;
            } else {
                carry_flag = true;
            }
        }

        if carry_flag {
            self.data_01.push(1);
        }
    }

    /// Accepts two integral primary literals and ensures that both data_01 vector dimensions (i.e nu of elements) are matched.
    pub fn _primlit_vec_elmnt_match(&mut self, right_nu: &mut SvPrimaryLiteralIntegral) {
        let left_size = self.data_01.len();
        let right_size = right_nu.data_01.len();

        if left_size > right_size {
            let diff: usize = left_size - right_size;

            for _x in 0..diff {
                right_nu.data_01.push(0);
                if right_nu.is_4state() {
                    right_nu.data_xz.as_mut().unwrap().push(0);
                }
            }
        } else if left_size < right_size {
            let diff: usize = right_size - left_size;

            for _x in 0..diff {
                self.data_01.push(0);
                if self.is_4state() {
                    self.data_xz.as_mut().unwrap().push(0);
                }
            }
        }
    }

    /// Receives an integral primary literal as an argument and deduces whether the stored value is -ve or not.
    ///
    /// # Examples
    ///
    /// Negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a.is_negative(), true);
    /// ```
    /// Positive value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a.is_negative(), false);
    /// ```
    /// Negative value with width > usize::BITS
    ///  ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a.is_negative(), true);
    /// ```
    /// Positive value with width > usize::BITS
    ///  ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a.is_negative(), false);
    /// ```
    pub fn is_negative(&self) -> bool {
        let mut zero = bit1b_0();
        zero.signed = true;

        self.lt(zero) == logic1b_1()
    }

    /// Receives an integral primary literal as an argument and deduces whether the stored value is zero or not.
    ///
    /// # Examples
    ///
    /// Zero with width = 1 bit
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: None,
    ///     size: 1,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a.is_zero(), true);
    /// ```
    /// Zero with width > usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a.is_zero(), true);
    /// ```
    /// Non-Zero with width > usize::BITS
    ///  ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a.is_zero(), false);
    /// ```
    pub fn is_zero(&self) -> bool {
        let mut zero = bit1b_0();
        zero.signed = true;

        self.case_eq(zero) == bit1b_1()
    }

    /// Deduces whether the primary literal is 4-state or not.
    pub fn is_4state(&self) -> bool {
        match self.data_xz.clone() {
            None => false,
            Some(_) => true,
        }
    }

    /// Receives an integral primary literal as an argument and deduces whether it contains X(s) or Z(s).
    pub fn contains_xz(&self) -> bool {
        if !self.is_4state() {
            return false;
        } else {
            for x in self.data_xz.as_ref().unwrap() {
                if x.leading_zeros() != usize::BITS {
                    return true;
                }
            }
        }

        false
    }

    /// Receives an integral primary literal and returns its contents in a 4-state integral primary literal.
    pub fn to_4state(&self) -> SvPrimaryLiteralIntegral {
        let mut ret = SvPrimaryLiteralIntegral {
            data_01: self.data_01.clone(),
            data_xz: Some(vec![0]),
            size: self.size,
            signed: self.signed,
        };

        if ret.data_01.len() != ret.data_xz.as_ref().unwrap().len() {
            for _x in 0..(ret.data_01.len() - ret.data_xz.as_ref().unwrap().len()) {
                let mut new_vec = ret.data_xz.clone().unwrap();
                new_vec.push(0);
                ret.data_xz = Some(new_vec);
            }
        }

        ret
    }

    /// Returns whether the MSB of data_01 is high. The size must be correctly specified.
    pub fn is_set_msb_01(&self) -> bool {
        let left_leading_zeros: usize =
            usize::BITS as usize - (self.size - (self.data_01.len() - 1) * usize::BITS as usize);

        if self.data_01[self.data_01.len() - 1].leading_zeros() as usize == left_leading_zeros {
            true
        } else {
            false
        }
    }

    /// Returns whether the MSB of data_xz is high. The size must be correctly specified.
    pub fn is_set_msb_xz(&self) -> bool {
        if self.is_4state() {
            let left_leading_zeros: usize = usize::BITS as usize
                - (self.size - (self.data_xz.as_ref().unwrap().len() - 1) * usize::BITS as usize);

            if self.data_xz.as_ref().unwrap()[self.data_xz.as_ref().unwrap().len() - 1]
                .leading_zeros() as usize
                == left_leading_zeros
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /** Accepts two signed integral primary literals and ensures that both are properly sign extended and matched to their data_01 dimensions.
    The correct final number of bits is set to both arguments. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Negative value with width = usize::BITS and positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with width = 2 * usize::BITS and positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with usize::BITS < width < 2 * usize::BITS and positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Positive value with width = usize::BITS and negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Negative value with width = usize::BITS and positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with width = 2 * usize::BITS and positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with usize::BITS < width < 2 * usize::BITS and positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with width = usize::BITS and positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: Some(vec![9223372036854775808, 18446744073709551615]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Value with with usize::BITS < width < 2 * usize::BITS and positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 18446744073709551615]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Value with with usize::BITS < width < 2 * usize::BITS (contains X/Z(s)) and positive value with width = usize::BITS (does not contain X/Z(s))
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let mut b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// a._matched_sign_extend(&mut b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: Some(vec![0, 18446744073709551615]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    pub fn _matched_sign_extend(&mut self, right_nu: &mut SvPrimaryLiteralIntegral) {
        if self.signed != true || right_nu.signed != true {
            panic!("Expected signed SvPrimaryLiterals but found unsigned!");
        }
        let left_neg: bool = self.is_negative();
        let right_neg: bool = right_nu.is_negative();

        let left_sign_x: bool = !self.is_set_msb_01() && self.is_set_msb_xz();
        let right_sign_x: bool = !right_nu.is_set_msb_01() && right_nu.is_set_msb_xz();

        let left_sign_z: bool = self.is_set_msb_01() && self.is_set_msb_xz();
        let right_sign_z: bool = right_nu.is_set_msb_01() && right_nu.is_set_msb_xz();

        self._primlit_vec_elmnt_match(right_nu);

        if left_neg || left_sign_z {
            let mut last_element: bool = false;

            for x in (0..self.data_01.len()).rev() {
                let left_leading = self.data_01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data_01[x] = self.data_01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        if left_sign_z || left_sign_x {
            let mut last_element: bool = false;

            for x in (0..self.data_xz.as_ref().unwrap().len()).rev() {
                let left_leading = self.data_xz.as_ref().unwrap()[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data_xz.as_mut().unwrap()[x] =
                        self.data_xz.as_ref().unwrap()[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        if right_neg || right_sign_z {
            let mut last_element: bool = false;

            for x in (0..right_nu.data_01.len()).rev() {
                let left_leading = right_nu.data_01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    right_nu.data_01[x] = right_nu.data_01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        if right_sign_z || right_sign_x {
            let mut last_element: bool = false;

            for x in (0..right_nu.data_xz.as_ref().unwrap().len()).rev() {
                let left_leading = right_nu.data_xz.as_ref().unwrap()[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    right_nu.data_xz.as_mut().unwrap()[x] =
                        right_nu.data_xz.as_ref().unwrap()[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        self.size = self.data_01.len() * usize::BITS as usize;
        right_nu.size = right_nu.data_01.len() * usize::BITS as usize;
    }

    /** Accepts two unsigned integral primary literals and ensures that both are properly zero extended and matched to their data_01 dimensions.
    The correct final number of bits is set to both arguments. */

    pub fn _matched_zero_extend(&mut self, right_nu: &mut SvPrimaryLiteralIntegral) {
        if self.signed == true || right_nu.signed == true {
            panic!("Expected unsigned SvPrimaryLiterals but found signed!");
        }

        self._primlit_vec_elmnt_match(right_nu);
        self.size = self.data_01.len() * usize::BITS as usize;
        right_nu.size = right_nu.data_01.len() * usize::BITS as usize;
    }

    /** Receives a signed integral primary literal and sign extends the value in the existing number of data_01 vector elements.
    The correct final number of bits is set to the argument. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 18446744073709551615],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 9223372036854775808]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 9223372036854775808]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._sign_extend();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 18446744073709551615]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    pub fn _sign_extend(&mut self) {
        if self.signed != true {
            panic!("Expected signed SvPrimaryLiteralIntegral but found unsigned!");
        }

        let left_neg: bool = self.is_negative();

        let left_sign_x: bool = !self.is_set_msb_01() && self.is_set_msb_xz();
        let left_sign_z: bool = self.is_set_msb_01() && self.is_set_msb_xz();

        if left_neg || left_sign_z {
            let mut last_element: bool = false;

            for x in (0..self.data_01.len()).rev() {
                let left_leading = self.data_01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data_01[x] = self.data_01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        if left_sign_z || left_sign_x {
            let mut last_element: bool = false;

            for x in (0..self.data_xz.as_ref().unwrap().len()).rev() {
                let left_leading = self.data_xz.as_ref().unwrap()[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data_xz.as_mut().unwrap()[x] =
                        self.data_xz.as_ref().unwrap()[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        self.size = self.data_01.len() * usize::BITS as usize;
    }

    /** Receives a signed integral primary literal and returns its opposite signed primary literal (i.e +ve -> -ve and vice versa).
    The correct final number of bits is set to the argument. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = -a;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Negative value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = -a;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775807],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = -a;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = -a;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = -a;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775807],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = -a;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    pub fn negate(&self) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        if ret.is_zero() {
            return ret;
        } else if ret.signed != true {
            panic!("Expected signed SvPrimaryLiteralIntegral but found unsigned!");
        }

        let from_negative: bool = ret.is_negative();
        ret = ret.inv();
        ret = ret + 1;
        let last_index = ret.data_01.len() - 1;

        if from_negative {
            ret.size = (usize::BITS as usize - ret.data_01[last_index].leading_zeros() as usize
                + 1)
                + (last_index) * usize::BITS as usize;

            if ret.data_01[last_index].leading_zeros() == 0 {
                ret.data_01.push(0);
            }
        } else {
            ret.size = (usize::BITS as usize - ret.data_01[last_index].leading_zeros() as usize)
                + (last_index) * usize::BITS as usize;
        }

        ret._minimum_width();

        if ret.is_4state() {
            ret.data_xz = ret.to_4state().data_xz;
        }

        ret
    }

    /** Receives a signed integral primary literal and returns a primary literal with its inverted value.
    The final number of bits remains the same as the original one.*/
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.inv();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775807, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904, 4611686018427387904],
    ///     data_xz: None,
    ///     size: 127,
    ///     signed: false,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.inv();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![13835058055282163711, 4611686018427387903],
    ///     data_xz: None,
    ///     size: 127,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.inv();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775807, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904, 4611686018427387904],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 127,
    ///     signed: false,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.inv();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![13835058055282163711, 4611686018427387903],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 127,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.inv();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775807, 0],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904, 4611686018427387904],
    ///     data_xz: Some(vec![1, 0]),
    ///     size: 127,
    ///     signed: false,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.inv();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![13835058055282163710, 4611686018427387903],
    ///     data_xz: Some(vec![1, 0]),
    ///     size: 127,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    pub fn inv(&self) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        let first_elmnt_bits: u32;
        if ret.size % usize::BITS as usize == 0 {
            first_elmnt_bits = usize::BITS;
        } else {
            first_elmnt_bits = ret.size as u32 % usize::BITS;
        }
        let remaining_bits = usize::BITS - first_elmnt_bits;
        let last_index = ret.data_01.len() - 1;

        for _x in 0..ret.size {
            if ret.is_4state()
                && (ret.data_xz.as_ref().unwrap()[last_index].leading_zeros() == remaining_bits)
            {
                if ret.data_01[last_index].leading_zeros() == remaining_bits {
                    ret.data_01[last_index] =
                        ret.data_01[last_index] - 2usize.pow(first_elmnt_bits - 1);
                }
            } else if ret.data_01[last_index].leading_zeros() == remaining_bits {
                ret.data_01[last_index] =
                    ret.data_01[last_index] - 2usize.pow(first_elmnt_bits - 1);
            } else {
                ret.data_01[last_index] =
                    ret.data_01[last_index] + 2usize.pow(first_elmnt_bits - 1);
            }

            ret = ret.ror(1);
        }

        ret
    }

    /** Receives the number of shift positions and implements logical shifting to the left.
    For each shift the total number of bits increments by 1 i.e. lsl works as 2^(positions) and the size of the integral primlit is dynamically adjusted.
    If an explicit range is defined, _truncate can be used afterwards.*/
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2, 2],
    ///     data_xz: None,
    ///     size: 130,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 4;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 4],
    ///     data_xz: None,
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2, 2],
    ///     data_xz: Some(vec![0, 0, 0]),
    ///     size: 130,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 4;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 4],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2],
    ///     data_xz: Some(vec![0, 2]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0, 1],
    ///     data_xz: Some(vec![0, 1, 1]),
    ///     size: 129,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp)
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![0, 2]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a << 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0, 0],
    ///     data_xz: Some(vec![0, 1, 1]),
    ///     size: 129,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    pub fn lsl(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        for _x in 0..n {
            let mut leading_one: bool = false;
            let mut leading_one_xz: bool = false;

            ret.size = ret.size + 1;

            for y in 0..ret.data_01.len() {
                let pre_mod = ret.data_01[y];

                if leading_one {
                    ret.data_01[y] = (ret.data_01[y] << 1) + 1;
                    leading_one = false;
                } else {
                    ret.data_01[y] = ret.data_01[y] << 1;
                }

                if pre_mod.leading_zeros() == 0 {
                    leading_one = true;
                }

                if ret.is_4state() {
                    let pre_mod = ret.data_xz.as_ref().unwrap()[y];

                    if leading_one_xz {
                        ret.data_xz.as_mut().unwrap()[y] =
                            (ret.data_xz.as_ref().unwrap()[y] << 1) + 1;
                        leading_one_xz = false;
                    } else {
                        ret.data_xz.as_mut().unwrap()[y] = ret.data_xz.as_ref().unwrap()[y] << 1;
                    }

                    if pre_mod.leading_zeros() == 0 {
                        leading_one_xz = true;
                    }
                }
            }

            if leading_one && leading_one_xz {
                ret.data_01.push(1);
                ret.data_xz.as_mut().unwrap().push(1);
            } else if leading_one {
                ret.data_01.push(1);
                if ret.is_4state() {
                    ret.data_xz.as_mut().unwrap().push(0);
                }
            } else if leading_one_xz {
                ret.data_01.push(0);
                ret.data_xz.as_mut().unwrap().push(1);
            } else if ret.signed && (ret.size > usize::BITS as usize * ret.data_01.len()) {
                ret.data_01.push(0);

                if ret.is_4state() {
                    ret.data_xz.as_mut().unwrap().push(0);
                }
            }
        }

        ret
    }

    /** Receives the number of shift positions and implements logical shifting to the right.
    The initial number of bits is preserved. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 3],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16140901064495857664, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 9223372036854775809],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![6917529027641081856, 2305843009213693952],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 4;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![288230376151711744],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 3],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16140901064495857664, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 9223372036854775809],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![6917529027641081856, 2305843009213693952],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 4;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![288230376151711744],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904, 0],
    ///     data_xz: Some(vec![4611686018427387904, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![2305843009213693952],
    ///     data_xz: Some(vec![2305843009213693952]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 2;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![2305843009213693952],
    ///     data_xz: Some(vec![2305843009213693952]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904, 4611686018427387904]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a >> 1;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    pub fn lsr(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        for _x in 0..n {
            let mut trailing_one: bool = false;
            let mut trailing_one_xz: bool = false;

            for y in (0..ret.data_01.len()).rev() {
                let pre_mod = ret.data_01[y];

                if trailing_one {
                    ret.data_01[y] = (ret.data_01[y] >> 1) + 2usize.pow(usize::BITS - 1);
                    trailing_one = false;
                } else {
                    ret.data_01[y] = ret.data_01[y] >> 1;
                }

                if pre_mod.trailing_zeros() == 0 {
                    trailing_one = true;
                }

                if ret.is_4state() {
                    let pre_mod = ret.data_xz.as_ref().unwrap()[y];

                    if trailing_one_xz {
                        ret.data_xz.as_mut().unwrap()[y] =
                            (ret.data_xz.as_ref().unwrap()[y] >> 1) + 2usize.pow(usize::BITS - 1);
                        trailing_one_xz = false;
                    } else {
                        ret.data_xz.as_mut().unwrap()[y] = ret.data_xz.as_ref().unwrap()[y] >> 1;
                    }

                    if pre_mod.trailing_zeros() == 0 {
                        trailing_one_xz = true;
                    }
                }
            }
        }

        ret
    }

    /** Receives the number of shift positions and shifts the value to the left without changing the number of bits.
    The dropped bits are shifted in the RHS of the value. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.rol(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.rol(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![2, 2],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.rol(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.rol(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![2, 2],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![9223372036854775808, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.rol(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 0],
    ///     data_xz: Some(vec![3, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.rol(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![2, 2],
    ///     data_xz: Some(vec![0, 2]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    pub fn rol(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        for _x in 0..n {
            let previous_size = ret.size;
            let leading_one: bool = ret.is_set_msb_01();
            let leading_one_xz: bool = ret.is_set_msb_xz();

            ret = ret.lsl(1);
            ret._truncate(previous_size);
            if leading_one {
                ret.data_01[0] = ret.data_01[0] + 1;
            }

            if leading_one_xz {
                ret.data_xz.as_mut().unwrap()[0] = ret.data_xz.as_ref().unwrap()[0] + 1;
            }
        }

        ret
    }

    /* Receives the number of shift positions and shifts the value to the right without changing the number of bits.
    The dropped bits are shifted in the LHS of the value. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 3],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.ror(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16140901064495857664, 1],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 9223372036854775809],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.ror(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![6917529027641081856, 6917529027641081856],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 3],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.ror(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16140901064495857664, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 9223372036854775809],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.ror(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![6917529027641081856, 6917529027641081856],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 3],
    ///     data_xz: Some(vec![1, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.ror(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16140901064495857664, 1],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    /// Value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 9223372036854775809],
    ///     data_xz: Some(vec![9223372036854775809, 9223372036854775809]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b: SvPrimaryLiteralIntegral = a.ror(2);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![6917529027641081856, 6917529027641081856],
    ///     data_xz: Some(vec![6917529027641081856, 6917529027641081856]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(b, exp);
    /// ```
    pub fn ror(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        let last_index = ret.data_01.len() - 1;
        let msb: u32;

        if ret.size % usize::BITS as usize == 0 {
            msb = usize::BITS;
        } else {
            msb = ret.size as u32 % usize::BITS;
        }

        for _x in 0..n {
            let trailing_one: bool = ret.data_01[0].trailing_zeros() == 0;
            let mut trailing_one_xz: bool = false;

            if ret.is_4state() {
                trailing_one_xz = ret.data_xz.as_ref().unwrap()[0].trailing_zeros() == 0;
            }

            ret = ret.lsr(1);

            if trailing_one {
                ret.data_01[last_index] = ret.data_01[last_index] + 2usize.pow(msb - 1);
            }

            if trailing_one_xz {
                ret.data_xz.as_mut().unwrap()[last_index] =
                    ret.data_xz.as_ref().unwrap()[last_index] + 2usize.pow(msb - 1);
            }
        }

        ret
    }

    /** Receives two integral primary literals, concatenates them (logically shifts left the LHS primlit by RHS primlit's size and adds them).
    Returns an integral SvPrimaryLiteralIntegral with the final value. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Value with width = usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![13835058055282163712, 4611686018427387904],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Value with width = usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 4],
    ///     data_xz: None,
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width = usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775809, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![13835058055282163712, 4611686018427387904],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Value with width = usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 4],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Value with width = usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Value with usize::BITS < width < 2 * usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904, 4611686018427387904]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Value with width = usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a.cat(b);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 4],
    ///     data_xz: Some(vec![0, 4]),
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    pub fn cat(&self, right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        ret = ret.lsl(right_nu.size);

        let mut left_nu: SvPrimaryLiteralIntegral = ret.clone();

        if left_nu.is_4state() || right_nu.is_4state() {
            let mut left_xz = SvPrimaryLiteralIntegral {
                data_01: left_nu.data_xz.as_ref().unwrap().clone(),
                data_xz: None,
                size: left_nu.size,
                signed: false,
            };

            let right_xz = SvPrimaryLiteralIntegral {
                data_01: right_nu.data_xz.as_ref().unwrap().clone(),
                data_xz: None,
                size: right_nu.size,
                signed: false,
            };

            left_xz._unsigned_primlit_add(right_xz.clone());
            left_nu.data_xz = Some(left_xz.data_01.clone());
        }

        ret._unsigned_primlit_add(right_nu.clone());
        ret.size = self.size + right_nu.size;
        ret.data_xz = left_nu.data_xz.clone();

        ret
    }

    /** Emulates the less than operator "<" as defined in 1800-2017 | 11.4.4 Relational operators */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with width = usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with width = usize::BITS and signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with width < usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with width = usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with width = usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Same unsigned value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Same signed positive value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with usize::BITS < width < 2 * usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 3],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with = usize::BITS and unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width < usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Value with width = usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Signed value with usize::BITS < width < 2 * usize::BITS and signed value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.lt(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    pub fn lt(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu = self.clone();

        if left_nu.contains_xz() || right_nu.contains_xz() {
            logic1b_x()
        } else if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            left_nu.lt(right_nu.clone())
        } else {
            if left_nu.signed {
                let left_nu_neg: bool = left_nu.is_set_msb_01();
                let right_nu_neg: bool = right_nu.is_set_msb_01();

                if left_nu_neg && !right_nu_neg {
                    logic1b_1()
                } else if !left_nu_neg && right_nu_neg {
                    logic1b_0()
                } else {
                    if left_nu_neg {
                        left_nu._matched_sign_extend(&mut right_nu);

                        for x in (0..left_nu.data_01.len()).rev() {
                            if left_nu.data_01[x] < right_nu.data_01[x] {
                                return logic1b_1();
                            }
                        }

                        logic1b_0()
                    } else {
                        left_nu.signed = false;
                        right_nu.signed = false;

                        left_nu.lt(right_nu.clone())
                    }
                }
            } else {
                left_nu._matched_zero_extend(&mut right_nu);

                for x in (0..left_nu.data_01.len()).rev() {
                    if left_nu.data_01[x] < right_nu.data_01[x] {
                        return logic1b_1();
                    }
                }

                logic1b_0()
            }
        }
    }

    /** Emulates the less than or equal operator "<=" as defined in 1800-2017 | 11.4.4 Relational operators */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with width = usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with width = usize::BITS and signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with width < usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with width = usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with width = usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Same unsigned value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Same signed positive value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with usize::BITS < width < 2 * usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 3],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with = usize::BITS and unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width < usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Value with width = usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Signed value with usize::BITS < width < 2 * usize::BITS and signed value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.le(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    pub fn le(&self, right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        if self.contains_xz() || right_nu.contains_xz() {
            logic1b_x()
        } else {
            let lt = self.lt(right_nu.clone());
            let logical_eq = self.logical_eq(right_nu.clone());

            if lt == logic1b_1() || logical_eq == logic1b_1() {
                return logic1b_1();
            }

            logic1b_0()
        }
    }

    /** Emulates the greater than operator ">" as defined in 1800-2017 | 11.4.4 Relational operators */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with width = usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with width = usize::BITS and signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with width < usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with width = usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with width = usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Same unsigned value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Same signed positive value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with usize::BITS < width < 2 * usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 3],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with = usize::BITS and unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width < usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Value with width = usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Signed value with usize::BITS < width < 2 * usize::BITS and signed value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.gt(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    pub fn gt(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu = self.clone();

        if left_nu.contains_xz() || right_nu.contains_xz() {
            logic1b_x()
        } else if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            left_nu.gt(right_nu.clone())
        } else {
            if left_nu.signed {
                let left_nu_neg: bool = left_nu.is_set_msb_01();
                let right_nu_neg: bool = right_nu.is_set_msb_01();

                if left_nu_neg && !right_nu_neg {
                    logic1b_0()
                } else if !left_nu_neg && right_nu_neg {
                    logic1b_1()
                } else {
                    left_nu._matched_sign_extend(&mut right_nu);

                    for x in (0..left_nu.data_01.len()).rev() {
                        if left_nu.data_01[x] > right_nu.data_01[x] {
                            return logic1b_1();
                        }
                    }

                    logic1b_0()
                }
            } else {
                left_nu._matched_zero_extend(&mut right_nu);

                for x in (0..left_nu.data_01.len()).rev() {
                    if left_nu.data_01[x] > right_nu.data_01[x] {
                        return logic1b_1();
                    }
                }

                logic1b_0()
            }
        }
    }

    /** Emulates the greater than or equal operator ">=" as defined in 1800-2017 | 11.4.4 Relational operators */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with width = usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with width = usize::BITS and signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed negative value with width < usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with width = usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Signed positive value with width = usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Same unsigned value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Same signed positive value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with usize::BITS < width < 2 * usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 3],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed negative value with = usize::BITS and unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width < usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Value with width = usize::BITS and value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Signed value with usize::BITS < width < 2 * usize::BITS and signed value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.ge(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    pub fn ge(&self, right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        if self.contains_xz() || right_nu.contains_xz() {
            logic1b_x()
        } else {
            let gt = self.gt(right_nu.clone());
            let logical_eq = self.logical_eq(right_nu.clone());

            if gt == logic1b_1() || logical_eq == logic1b_1() {
                return logic1b_1();
            }

            logic1b_0()
        }
    }

    /** Emulates the case equality operator "===" as defined in 1800-2017 | 11.4.5 Equality operators */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed positive value with width = usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed negative value with width = usize::BITS and signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed negative value with width < usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed negative value with width = usize::BITS and signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed positive value with width = usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Same unsigned value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Same signed positive value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Signed negative value with usize::BITS < width < 2 * usize::BITS and signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 3],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Signed negative value with = usize::BITS and unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width = usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Value with width < usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Two signed values both with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Two signed values with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Two signed values with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Two signed values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed value with usize::BITS < width < 2 * usize::BITS and signed value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![9223372036854775808, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    /// Two signed values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_0());
    /// ```
    /// Signed value with width = usize::BITS and signed values with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![9223372036854775808, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.case_eq(b);
    ///
    /// assert_eq!(c, bit1b_1());
    /// ```
    pub fn case_eq(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu = self.clone();
        if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            left_nu.case_eq(right_nu.clone())
        } else if left_nu.contains_xz() != right_nu.contains_xz() {
            bit1b_0()
        } else if left_nu.contains_xz() && right_nu.contains_xz() {
            if left_nu.signed {
                left_nu._matched_sign_extend(&mut right_nu);
            } else {
                left_nu._matched_zero_extend(&mut right_nu);
            }

            let data_01 = left_nu.data_01 == right_nu.data_01;
            let data_xz = left_nu.data_xz.as_ref().unwrap() == right_nu.data_xz.as_ref().unwrap();

            if data_01 && data_xz {
                return bit1b_1();
            }
            bit1b_0()
        } else {
            if left_nu.signed {
                left_nu._matched_sign_extend(&mut right_nu);
            } else {
                left_nu._matched_zero_extend(&mut right_nu);
            }

            if left_nu.data_01 == right_nu.data_01 {
                return bit1b_1();
            }

            bit1b_0()
        }
    }

    /** Emulates the logical equality operator "==" as defined in 1800-2017 | 11.4.5 Equality operators */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Two signed values both with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Same signed positive value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width = usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Value with width < usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Two signed values both with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two signed values with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two signed values with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775808, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.logical_eq(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    pub fn logical_eq(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu = self.clone();

        if left_nu.contains_xz() || right_nu.contains_xz() {
            logic1b_x()
        } else if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            left_nu.logical_eq(right_nu.clone())
        } else {
            left_nu.case_eq(right_nu.clone()).to_4state()
        }
    }

    /** Emulates the wildcard equality operator "==?" as defined in 1800-2017 | 11.4.6 Wildcard equality operators */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Two signed values both with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Same signed positive value twice but with different widths
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Value with width = usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_0());
    /// ```
    /// Value with width < usize::BITS and value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS and signed positive value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Two signed values both with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    /// Two unsigned values both with width <= usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Two signed values with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![1, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_1());
    /// ```
    /// Two signed values with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775809, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![9223372036854775809, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let c = a.wildcard_eq(b);
    ///
    /// assert_eq!(c, logic1b_x());
    /// ```
    pub fn wildcard_eq(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu = self.clone();

        if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            left_nu.wildcard_eq(right_nu.clone())
        } else if !right_nu.contains_xz() {
            left_nu.logical_eq(right_nu.clone())
        } else {
            if left_nu.signed {
                left_nu._matched_sign_extend(&mut right_nu);
            } else {
                left_nu._matched_zero_extend(&mut right_nu);
            }
            let last_index = right_nu.data_01.len() - 1;
            for _x in 0..left_nu.size {
                let left_msb_x: bool = !left_nu.is_set_msb_01() && left_nu.is_set_msb_xz();
                let left_msb_z: bool = left_nu.is_set_msb_01() && left_nu.is_set_msb_xz();
                let left_msb_0: bool = !left_nu.is_set_msb_01() && !left_nu.is_set_msb_xz();
                let left_msb_1: bool = left_nu.is_set_msb_01() && !left_nu.is_set_msb_xz();

                let right_msb_x: bool = !right_nu.is_set_msb_01() && right_nu.is_set_msb_xz();
                let right_msb_z: bool = right_nu.is_set_msb_01() && right_nu.is_set_msb_xz();

                if right_msb_x {
                    if left_msb_z {
                        right_nu.data_01[last_index] =
                            right_nu.data_01[last_index] + 2usize.pow(usize::BITS - 1);
                    } else if left_msb_1 {
                        right_nu.data_01[last_index] =
                            right_nu.data_01[last_index] + 2usize.pow(usize::BITS - 1);
                        right_nu.data_xz.as_mut().unwrap()[last_index] =
                            right_nu.data_xz.as_ref().unwrap()[last_index]
                                - 2usize.pow(usize::BITS - 1);
                    } else if left_msb_0 {
                        right_nu.data_xz.as_mut().unwrap()[last_index] =
                            right_nu.data_xz.as_ref().unwrap()[last_index]
                                - 2usize.pow(usize::BITS - 1);
                    }
                } else if right_msb_z {
                    if left_msb_x {
                        right_nu.data_01[last_index] =
                            right_nu.data_01[last_index] - 2usize.pow(usize::BITS - 1);
                    } else if left_msb_1 {
                        right_nu.data_xz.as_mut().unwrap()[last_index] =
                            right_nu.data_xz.as_ref().unwrap()[last_index]
                                - 2usize.pow(usize::BITS - 1);
                    } else if left_msb_0 {
                        right_nu.data_01[last_index] =
                            right_nu.data_01[last_index] - 2usize.pow(usize::BITS - 1);
                        right_nu.data_xz.as_mut().unwrap()[last_index] =
                            right_nu.data_xz.as_ref().unwrap()[last_index]
                                - 2usize.pow(usize::BITS - 1);
                    }
                }

                left_nu = left_nu.rol(1);
                right_nu = right_nu.rol(1);
            }

            left_nu.logical_eq(right_nu)
        }
    }

    /** Receives a signed or unsigned integral primary literal and deduces an equivalent representation with the minimum number of bits required.
    The correct final number of bits is set to the argument. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![65533],
    ///     data_xz: None,
    ///     size: 16,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![5],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed positive value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed value = 0 with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: None,
    ///     size: 1,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 0],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value = 0 with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: None,
    ///     size: 1,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![65533],
    ///     data_xz: Some(vec![0]),
    ///     size: 16,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![5],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed negative value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed positive value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed value = 0 with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![0]),
    ///     size: 1,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value = 0 with width = 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._minimum_width();
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![0]),
    ///     size: 1,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    pub fn _minimum_width(&mut self) {
        if !self.signed {
            if self.is_zero() {
                for _x in 0..self.data_01.len() {
                    let last_index = self.data_01.len() - 1;
                    self.data_01.remove(last_index);
                }
                self.data_01.push(0);
                self.size = 1;
            } else {
                for _x in 0..self.data_01.len() {
                    let last_index = self.data_01.len() - 1;
                    if self.data_01[last_index] == 0 {
                        self.data_01.remove(last_index);
                    }
                }

                self.size = (usize::BITS as usize
                    - self.data_01[self.data_01.len() - 1].leading_zeros() as usize)
                    + (self.data_01.len() - 1) * usize::BITS as usize;
            }
        } else {
            let mut min_num_found: bool = false;
            let mut vec_elements_to_rm: usize = 0;

            if self.is_negative() {
                for x in (0..self.data_01.len()).rev() {
                    while !min_num_found {
                        let pre_leading = self.data_01[x].leading_zeros();

                        let minimized_value: usize =
                            self.data_01[x] - 2usize.pow(usize::BITS - pre_leading - 1);
                        let post_leading = minimized_value.leading_zeros();

                        if post_leading == usize::BITS {
                            if x == 0 || self.data_01[x - 1].leading_zeros() != 0 {
                                min_num_found = true;
                                break;
                            }
                        }

                        if post_leading != (pre_leading + 1) {
                            min_num_found = true;
                            break;
                        } else {
                            self.data_01[x] = minimized_value;
                            self.size = self.size - 1;

                            if post_leading == usize::BITS {
                                vec_elements_to_rm = vec_elements_to_rm + 1;
                                break;
                            }
                        }
                    }
                }

                for _x in 0..vec_elements_to_rm {
                    let last_index = self.data_01.len() - 1;
                    self.data_01.remove(last_index);
                }
            } else if self.is_zero() {
                for _x in 0..self.data_01.len() {
                    let last_index = self.data_01.len() - 1;
                    self.data_01.remove(last_index);
                }
                self.data_01.push(0);
                self.size = 1;
            } else {
                for _x in 0..self.data_01.len() {
                    let last_index = self.data_01.len() - 1;
                    if self.data_01[last_index] == 0 {
                        self.data_01.remove(last_index);
                    }
                }

                let last_index = self.data_01.len() - 1;
                if self.data_01[last_index].leading_zeros() == 0 {
                    self.data_01.push(0);
                }

                self.size = (usize::BITS as usize
                    - self.data_01[self.data_01.len() - 1].leading_zeros() as usize
                    + 1)
                    + (self.data_01.len() - 1) * usize::BITS as usize;
            }
        }

        if self.is_4state() && (self.data_01.len() < self.data_xz.as_ref().unwrap().len()) {
            for _x in 0..(self.data_xz.as_ref().unwrap().len() - self.data_01.len()) {
                let last_index = self.data_01.len() - 1;
                self.data_xz.as_mut().unwrap().remove(last_index);
            }
        }
    }

    /** Receives the number of bits in which an integral primary literal should be truncated.
    The correct final number of bits is set but the signedness doesn't change. */
    /// # Examples
    ///
    /// ## 2-State Primary Literals
    ///
    /// Signed negative value with width = usize::BITS truncated to 64 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._truncate(64);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed negative value with width = usize::BITS truncated to 5 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387905, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._truncate(5);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1],
    ///     data_xz: None,
    ///     size: 5,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = usize::BITS truncated to 69 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775809],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._truncate(69);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 69,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = usize::BITS truncated to 1 bit
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1, 0],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._truncate(1);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1],
    ///     data_xz: None,
    ///     size: 1,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (No X/Z(s))
    ///
    /// Signed negative value with width = usize::BITS truncated to 64 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._truncate(64);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed negative value with width = usize::BITS truncated to 5 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387905, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._truncate(5);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = usize::BITS truncated to 69 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775809],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._truncate(69);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 69,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = usize::BITS truncated to 1 bit
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._truncate(1);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1],
    ///     data_xz: Some(vec![0]),
    ///     size: 1,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals (Containing X/Z(s))
    ///
    /// Signed value with width = usize::BITS truncated to 64 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._truncate(64);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Signed value with width = usize::BITS truncated to 5 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387905, 9223372036854775808],
    ///     data_xz: Some(vec![4611686018427387905, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// a._truncate(5);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1],
    ///     data_xz: Some(vec![1]),
    ///     size: 5,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = usize::BITS truncated to 69 bits
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775809],
    ///     data_xz: Some(vec![0, 9223372036854775809]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._truncate(69);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 69,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    /// Unsigned value with width = usize::BITS truncated to 1 bit
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let mut a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1, 0],
    ///     data_xz: Some(vec![1, 0]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// a._truncate(1);
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1],
    ///     data_xz: Some(vec![1]),
    ///     size: 1,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(a, exp);
    /// ```
    pub fn _truncate(&mut self, size: usize) {
        if size == 0 {
            panic!("Cannot truncate the value to zero bits!");
        } else if self.size >= size {
            let elmnts_to_be_rm: usize;
            let bits_to_be_rm: usize;

            if (size % usize::BITS as usize) == 0 {
                elmnts_to_be_rm = self.data_01.len() - size / usize::BITS as usize;
                bits_to_be_rm = 0;
            } else {
                elmnts_to_be_rm = self.data_01.len() - (size / usize::BITS as usize) - 1;
                bits_to_be_rm = usize::BITS as usize - size % usize::BITS as usize;
            }

            for _x in 0..elmnts_to_be_rm {
                let last_index = self.data_01.len() - 1;
                self.data_01.remove(last_index);
            }

            if bits_to_be_rm != 0 {
                let last_index = self.data_01.len() - 1;
                for x in
                    ((usize::BITS as usize - bits_to_be_rm + 1)..(usize::BITS as usize + 1)).rev()
                {
                    if self.data_01[last_index].leading_zeros() == (usize::BITS - x as u32) {
                        self.data_01[last_index] =
                            self.data_01[last_index] - 2usize.pow(x as u32 - 1);
                    }
                }
            }

            if self.is_4state() {
                let elmnts_to_be_rm: usize;
                let bits_to_be_rm: usize;

                if (size % usize::BITS as usize) == 0 {
                    elmnts_to_be_rm =
                        self.data_xz.as_ref().unwrap().len() - size / usize::BITS as usize;
                    bits_to_be_rm = 0;
                } else {
                    elmnts_to_be_rm =
                        self.data_xz.as_ref().unwrap().len() - (size / usize::BITS as usize) - 1;
                    bits_to_be_rm = usize::BITS as usize - size % usize::BITS as usize;
                }

                for _x in 0..elmnts_to_be_rm {
                    let last_index = self.data_xz.as_ref().unwrap().len() - 1;
                    self.data_xz.as_mut().unwrap().remove(last_index);
                }

                if bits_to_be_rm != 0 {
                    let last_index = self.data_xz.as_ref().unwrap().len() - 1;
                    for x in ((usize::BITS as usize - bits_to_be_rm + 1)
                        ..(usize::BITS as usize + 1))
                        .rev()
                    {
                        if self.data_xz.as_ref().unwrap()[last_index].leading_zeros()
                            == (usize::BITS - x as u32)
                        {
                            self.data_xz.as_mut().unwrap()[last_index] =
                                self.data_xz.as_ref().unwrap()[last_index]
                                    - 2usize.pow(x as u32 - 1);
                        }
                    }
                }
            }

            self.size = size;
        } else {
            panic!("The original number of bits is smaller than the requested one!");
        }
    }

    /// # Examples
    ///
    /// ## 2-State Primary Literals - Signed Addition
    ///
    /// Signed negative value with width = usize::BITS added with itself
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width = 2 * usize::BITS added with a signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 9223372036854775808, 1],
    ///     data_xz: None,
    ///     size: 129,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width < usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width = usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![13835058055282163712, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width = usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width = 2 * usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904, 4611686018427387904],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 4611686018427387904, 0],
    ///     data_xz: None,
    ///     size: 129,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 2-State Primary Literals - Signed Unsigned Addition
    ///
    /// Signed negative value with width = usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width = usize::BITS added with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 2-State Primary Literals - Unsigned Addition
    ///
    /// Unsigned value with width = usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width < usize::BITS added with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: None,
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width = 2 * usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 9223372036854775809, 0],
    ///     data_xz: None,
    ///     size: 129,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Addition (No X/Z(s))
    ///
    /// Signed negative value with width = usize::BITS added with itself
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width = 2 * usize::BITS added with a signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 9223372036854775808, 1],
    ///     data_xz: Some(vec![0, 0, 0]),
    ///     size: 129,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width < usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width = usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![13835058055282163712, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width = usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width = 2 * usize::BITS added with a signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904, 4611686018427387904],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 4611686018427387904, 0],
    ///     data_xz: Some(vec![0, 0, 0]),
    ///     size: 129,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Unsigned Addition (No X/Z(s))
    ///
    /// Signed negative value with width = usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width = usize::BITS added with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Unsigned Addition (No X/Z(s))
    ///
    /// Unsigned value with width = usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 1],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width < usize::BITS added with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width = 2 * usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 128,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 9223372036854775809, 0],
    ///     data_xz: Some(vec![0, 0, 0]),
    ///     size: 129,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Addition (Containing X/Z(s))
    ///
    /// Signed value with width = usize::BITS added with signed negative value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed value with width = usize::BITS added with signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed value with width = usize::BITS added with signed positive value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 1]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Unsigned Addition (Containing X/Z(s))
    ///
    /// Signed negative value with width = usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 1]),
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width < usize::BITS added with a signed positive value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![7],
    ///     data_xz: Some(vec![3]),
    ///     size: 3,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![15],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![63]),
    ///     size: 6,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Unsigned Addition (Containing X/Z(s))
    ///
    /// Unsigned value with width = usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 1]),
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width = usize::BITS added with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![9223372036854775808]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 1]),
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width < usize::BITS added with an unsigned value with usize::BITS < width < 2 * usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 63,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4611686018427387904],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a + b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 1]),
    ///     size: 65,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    pub fn add_primlit(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        if ret.is_4state() != right_nu.is_4state() {
            if !ret.is_4state() {
                ret = ret.to_4state();
            } else {
                right_nu = right_nu.to_4state();
            }
        }

        if !ret.contains_xz() && !right_nu.contains_xz() {
            // Possible carry out from the MSB
            let final_num_bits: usize;
            let elmnts_sign_extension: usize;

            if ret.size > right_nu.size {
                final_num_bits = ret.size + 1;
                elmnts_sign_extension = ret.data_01.len() + 1;
            } else {
                final_num_bits = right_nu.size + 1;
                elmnts_sign_extension = right_nu.data_01.len() + 1;
            }

            if ret.signed == false || right_nu.signed == false {
                ret.signed = false;
            } else {
                let mut matched_prim_lit = bit1b_0();
                matched_prim_lit.signed = true;
                for _x in 0..(elmnts_sign_extension - 1) {
                    matched_prim_lit.data_01.push(0);
                }
                matched_prim_lit.size = elmnts_sign_extension * usize::BITS as usize;

                ret._matched_sign_extend(&mut matched_prim_lit);
                right_nu._matched_sign_extend(&mut matched_prim_lit);
            }

            ret._unsigned_primlit_add(right_nu.clone());

            if ret.signed {
                ret._truncate(final_num_bits);
            } else {
                ret.size = final_num_bits;
                if (ret.data_01.len() * usize::BITS as usize) < final_num_bits {
                    ret.data_01.push(0);
                }
            }

            if ret.is_4state() {
                ret.data_xz = ret.to_4state().data_xz;
            }

            ret
        } else {
            if ret.size < right_nu.size {
                ret.size = right_nu.size;
            }

            // Possible carry out from the MSB
            let final_num_bits = ret.size + 1;

            ret = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: !(ret.signed == false || right_nu.signed == false),
                size: 1,
            };

            let x_primlit = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: ret.signed,
                size: 1,
            };

            for _x in 0..(final_num_bits - 1) {
                ret = ret.cat(x_primlit.clone());
            }

            ret
        }
    }

    pub fn mul_unsigned(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral;
        let mut left_nu: SvPrimaryLiteralIntegral = self.clone();
        let mut add_ver: Vec<SvPrimaryLiteralIntegral> = Vec::new();

        for x in 0..right_nu.size {
            if right_nu.data_01[0].trailing_zeros() == 0 {
                if x == 0 {
                    add_ver.push(left_nu.clone());
                } else {
                    left_nu = left_nu.lsl(1);
                    add_ver.push(left_nu.clone());
                }
            } else if x != 0 {
                left_nu = left_nu.lsl(1);
            }

            right_nu = right_nu.lsr(1);
        }
        ret = SvPrimaryLiteralIntegral {
            data_01: vec![0],
            data_xz: None,
            signed: false,
            size: 1,
        };

        for y in 0..add_ver.len() {
            ret = ret.add_primlit(add_ver[y].clone());
        }

        ret
    }

    /// # Examples
    ///
    /// ## 2-State Primary Literals - Signed Multiplication
    ///
    /// Signed negative value with width < usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 2,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 5,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width = usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2],
    ///     data_xz: None,
    ///     size: 67,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width < usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![52],
    ///     data_xz: None,
    ///     size: 6,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 14],
    ///     data_xz: None,
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width < usize::BITS mult/ed with signed positive value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![12],
    ///     data_xz: None,
    ///     size: 7,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS mult/ed with signed positive value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: None,
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2],
    ///     data_xz: None,
    ///     size: 69,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 2-State Primary Literals - Signed Unsigned Multiplication
    ///
    /// Unsigned value with width < usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![12],
    ///     data_xz: None,
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width = usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2],
    ///     data_xz: None,
    ///     size: 67,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 2-State Primary Literals - Unsigned Multiplication
    ///
    /// Unsigned value with width < usize::BITS mult/ed with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![12],
    ///     data_xz: None,
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width < usize::BITS mult/ed with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![8],
    ///     data_xz: None,
    ///     size: 4,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: None,
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 4],
    ///     data_xz: None,
    ///     size: 68,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with 2 * usize::BITS < width < 3 * usize::BITS mult/ed with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1, 9223372036854775808, 9223372036854775808],
    ///     data_xz: None,
    ///     size: 192,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16],
    ///     data_xz: None,
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16, 0, 8, 8],
    ///     data_xz: None,
    ///     size: 197,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Multiplication (No X/Z(s))
    ///
    /// Signed negative value with width < usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 2,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed negative value with width = usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 67,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width < usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![52],
    ///     data_xz: Some(vec![0]),
    ///     size: 6,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 14],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with width < usize::BITS mult/ed with signed positive value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![12],
    ///     data_xz: Some(vec![0]),
    ///     size: 7,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS mult/ed with signed positive value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 69,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Unsigned Multiplication (No X/Z(s))
    ///
    /// Unsigned value with width < usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![12],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width = usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 2],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 67,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Unsigned Multiplication (No X/Z(s))
    ///
    /// Unsigned value with width < usize::BITS mult/ed with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![12],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width < usize::BITS mult/ed with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![8],
    ///     data_xz: Some(vec![0]),
    ///     size: 4,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 4],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 68,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with 2 * usize::BITS < width < 3 * usize::BITS mult/ed with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1, 9223372036854775808, 9223372036854775808],
    ///     data_xz: Some(vec![0, 0, 0]),
    ///     size: 192,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16, 0, 8, 8],
    ///     data_xz: Some(vec![0, 0, 0, 0]),
    ///     size: 197,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Multiplication (Containing X/Z(s))
    ///
    /// Signed negative value with width < usize::BITS mult/ed with signed value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![0]),
    ///     size: 2,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![8],
    ///     data_xz: Some(vec![4]),
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![63]),
    ///     size: 6,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed value with width = usize::BITS mult/ed with signed positive value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![4611686018427387904]),
    ///     size: 64,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 7]),
    ///     size: 67,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed value with width < usize::BITS mult/ed with signed value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![1]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![3]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![63]),
    ///     size: 6,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed positive value with usize::BITS < width < 2 * usize::BITS mult/ed with signed value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0]),
    ///     size: 65,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![1]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 15]),
    ///     size: 68,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed value with width < usize::BITS mult/ed with signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![1]),
    ///     size: 2,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![127]),
    ///     size: 7,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Signed value with usize::BITS < width < 2 * usize::BITS mult/ed with signed value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 1]),
    ///     size: 66,
    ///     signed: true,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![0]),
    ///     size: 4,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 63]),
    ///     size: 70,
    ///     signed: true,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Signed Unsigned Multiplication (Containing X/Z(s))
    ///
    /// Unsigned value with width < usize::BITS mult/ed with a signed negative value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: Some(vec![3]),
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: None,
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![31]),
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width = usize::BITS mult/ed with a signed value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![4]),
    ///     size: 3,
    ///     signed: true,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 7]),
    ///     size: 67,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    ///
    /// ## 4-State Primary Literals - Unsigned Multiplication (Containing X/Z(s))
    ///
    /// Unsigned value with width < usize::BITS mult/ed with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![3],
    ///     data_xz: None,
    ///     size: 2,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![4],
    ///     data_xz: Some(vec![4]),
    ///     size: 3,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![31]),
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with width < usize::BITS mult/ed with an unsigned value with width = usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0],
    ///     data_xz: Some(vec![8]),
    ///     size: 4,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![9223372036854775808],
    ///     data_xz: Some(vec![0]),
    ///     size: 64,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 15]),
    ///     size: 68,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    /// Unsigned value with 2 * usize::BITS < width < 3 * usize::BITS mult/ed with an unsigned value with width < usize::BITS
    /// ```
    /// # use svdata::sv_primlit_integral::*;
    /// let a = SvPrimaryLiteralIntegral {
    ///     data_01: vec![1, 9223372036854775808, 0],
    ///     data_xz: Some(vec![0, 0, 9223372036854775808]),
    ///     size: 192,
    ///     signed: false,
    /// };
    ///
    /// let b = SvPrimaryLiteralIntegral {
    ///     data_01: vec![16],
    ///     data_xz: Some(vec![0]),
    ///     size: 5,
    ///     signed: false,
    /// };
    ///
    /// let c: SvPrimaryLiteralIntegral = a * b;
    ///
    /// let exp = SvPrimaryLiteralIntegral {
    ///     data_01: vec![0, 0, 0, 0],
    ///     data_xz: Some(vec![18446744073709551615, 18446744073709551615, 18446744073709551615, 31]),
    ///     size: 197,
    ///     signed: false,
    /// };
    ///
    /// assert_eq!(c, exp);
    /// ```
    pub fn mult(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu: SvPrimaryLiteralIntegral = self.clone();
        let mut ret: SvPrimaryLiteralIntegral;

        if left_nu.is_4state() != right_nu.is_4state() {
            if !left_nu.is_4state() {
                left_nu = left_nu.to_4state();
            } else {
                right_nu = right_nu.to_4state();
            }
        }

        let final_num_bits: usize = left_nu.size + right_nu.size;
        let elmnts_sign_extension: usize = left_nu.data_01.len() + right_nu.data_01.len();

        if !left_nu.contains_xz() && !right_nu.contains_xz() {
            if left_nu.signed && right_nu.signed {
                let mut matched_prim_lit = bit1b_0();
                matched_prim_lit.signed = true;
                for _x in 0..(elmnts_sign_extension - 1) {
                    matched_prim_lit.data_01.push(0);
                }
                matched_prim_lit.size = elmnts_sign_extension * usize::BITS as usize;

                left_nu._matched_sign_extend(&mut matched_prim_lit);
                right_nu._matched_sign_extend(&mut matched_prim_lit);
            }

            ret = left_nu.mul_unsigned(right_nu.clone());
            if ret.size > final_num_bits {
                ret._truncate(final_num_bits);
            } else {
                ret.size = final_num_bits;
                // Due to the addition within unsigned_mult we can always expect that ret.data_01.len() is sufficient enough for final_num_bits.
            }

            ret.signed = left_nu.signed && right_nu.signed;

            if ret.is_4state() {
                ret.data_xz = ret.to_4state().data_xz;
            }
        } else {
            let final_num_bits = left_nu.size + right_nu.size;

            ret = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: !(left_nu.signed == false || right_nu.signed == false),
                size: 1,
            };

            let x_primlit = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: ret.signed,
                size: 1,
            };

            for _x in 0..(final_num_bits - 1) {
                ret = ret.cat(x_primlit.clone());
            }
        }

        ret
    }
}

/** Converts a usize into a 2-state signed primary literal. Width is set by deafult to usize::BITS */
/// # Examples
///
/// Signed positive value
/// ```
/// # use svdata::sv_primlit_integral::*;
/// let a: SvPrimaryLiteralIntegral = usize_to_primlit(4611686018427387904);
///
/// let exp = SvPrimaryLiteralIntegral {
///     data_01: vec![4611686018427387904],
///     data_xz: None,
///     size: 64,
///     signed: true,
/// };
///
/// assert_eq!(a, exp);
/// ```
/// Signed negative value
/// ```
/// # use svdata::sv_primlit_integral::*;
/// let a: SvPrimaryLiteralIntegral = usize_to_primlit(9223372036854775808);
///
/// let exp = SvPrimaryLiteralIntegral {
///     data_01: vec![9223372036854775808],
///     data_xz: None,
///     size: 64,
///     signed: true,
/// };
///
/// assert_eq!(a, exp);
/// ```
pub fn usize_to_primlit(value: usize) -> SvPrimaryLiteralIntegral {
    let mut ret = SvPrimaryLiteralIntegral {
        data_01: vec![value],
        data_xz: None,
        size: usize::BITS as usize,
        signed: true,
    };

    ret._minimum_width();

    ret
}

pub fn bit1b_0() -> SvPrimaryLiteralIntegral {
    SvPrimaryLiteralIntegral {
        data_01: vec![0],
        data_xz: None,
        size: 1,
        signed: false,
    }
}

pub fn bit1b_1() -> SvPrimaryLiteralIntegral {
    SvPrimaryLiteralIntegral {
        data_01: vec![1],
        data_xz: None,
        size: 1,
        signed: false,
    }
}

pub fn logic1b_0() -> SvPrimaryLiteralIntegral {
    SvPrimaryLiteralIntegral {
        data_01: vec![0],
        data_xz: Some(vec![0]),
        size: 1,
        signed: false,
    }
}

pub fn logic1b_1() -> SvPrimaryLiteralIntegral {
    SvPrimaryLiteralIntegral {
        data_01: vec![1],
        data_xz: Some(vec![0]),
        size: 1,
        signed: false,
    }
}

pub fn logic1b_x() -> SvPrimaryLiteralIntegral {
    SvPrimaryLiteralIntegral {
        data_01: vec![0],
        data_xz: Some(vec![1]),
        size: 1,
        signed: false,
    }
}

pub fn _logic1b_z() -> SvPrimaryLiteralIntegral {
    SvPrimaryLiteralIntegral {
        data_01: vec![1],
        data_xz: Some(vec![1]),
        size: 1,
        signed: false,
    }
}

impl fmt::Display for SvPrimaryLiteralIntegral {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "NumBits: {}", self.size)?;
        writeln!(f, "Signed: {}", self.signed)?;
        writeln!(f, "4State: {}", self.is_4state())?;

        let mut string_vec: Vec<String> = Vec::new();
        let mut s: String = String::new();
        let mut mod_primlit = self.clone();
        let first_elmnt_bits: u32;

        if mod_primlit.size % usize::BITS as usize == 0 {
            first_elmnt_bits = usize::BITS;
        } else {
            first_elmnt_bits = mod_primlit.size as u32 % usize::BITS;
        }
        let remaining_bits = usize::BITS - first_elmnt_bits;

        let last_index = mod_primlit.data_01.len() - 1;

        for _x in 0..first_elmnt_bits {
            if mod_primlit.is_4state()
                && (mod_primlit.data_xz.as_ref().unwrap()[last_index].leading_zeros()
                    == remaining_bits)
            {
                if mod_primlit.data_01[last_index].leading_zeros() == remaining_bits {
                    s.push('Z');
                } else {
                    s.push('X');
                }
            } else if mod_primlit.data_01[last_index].leading_zeros() == remaining_bits {
                s.push('1');
            } else {
                s.push('0');
            }

            mod_primlit = mod_primlit.rol(1);
        }

        string_vec.push(s);

        if self.data_01.len() > 1 {
            for x in (0..(self.data_01.len() - 1)).rev() {
                let mut mod_primlit = self.clone();
                let mut s: String = String::new();

                for _y in 0..usize::BITS {
                    if mod_primlit.is_4state()
                        && (mod_primlit.data_xz.as_ref().unwrap()[x].leading_zeros() == 0)
                    {
                        if mod_primlit.data_01[x].leading_zeros() == 0 {
                            s.push('Z');
                        } else {
                            s.push('X');
                        }
                    } else if mod_primlit.data_01[x].leading_zeros() == 0 {
                        s.push('1');
                    } else {
                        s.push('0');
                    }

                    mod_primlit = mod_primlit.rol(1);
                }

                string_vec.push(s);
            }
        }

        write!(f, "Data: ")?;
        for x in string_vec {
            writeln!(f, "{}", x)?;
        }

        write!(f, "")
    }
}

impl Add for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_primlit(rhs.clone())
    }
}

impl Add<usize> for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        let rhs = usize_to_primlit(rhs);
        self.add_primlit(rhs.clone())
    }
}

impl Mul for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.mult(rhs.clone())
    }
}

impl Shl<usize> for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        self.lsl(rhs)
    }
}

impl Shr<usize> for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        self.lsr(rhs)
    }
}

impl Neg for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn neg(self) -> Self {
        if self.contains_xz() {
            logic1b_x()
        } else {
            self.negate()
        }
    }
}
