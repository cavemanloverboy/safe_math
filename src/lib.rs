use std::ops::*;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Token {
    value: u64,
    decimals: u8,
}

impl Add for Token {

    type Output = Token;

    fn add(self, rhs: Self) -> Self::Output {
        
        // This checks whether we are going to scale rhs or self
        let scale_rhs: bool = self.decimals > rhs.decimals;

        // Calculate scale factor (will just be 0 if diff=0)
        let diff: u32 = self.decimals
            .max(rhs.decimals)
            .sub(self.decimals.min(rhs.decimals)) as u32;
        let scale = 10_u64.pow(diff);
        
        // Calculate new value after normalizing properly
        let new_value = {
            if scale_rhs {
                rhs.value.mul(scale).add(self.value)
            } else { 
                self.value.mul(scale).add(rhs.value)
            }
        };
        let new_decimals = self.decimals.max(rhs.decimals);

        // Construct result
        Token {
            value: new_value,
            decimals: new_decimals,
        }
    }
}

impl Sub for Token {

    type Output = Token;

    fn sub(self, rhs: Self) -> Self::Output {
        
        // This checks whether we are going to scale rhs or self
        let scale_rhs: bool = self.decimals > rhs.decimals;

        // Calculate scale factor (will just be 0 if diff=0)
        let diff: u32 = self.decimals
            .max(rhs.decimals)
            .sub(self.decimals.min(rhs.decimals)) as u32;
        let scale = 10_u64.pow(diff);
        
        // Calculate new value after normalizing properly
        let new_value = {
            if scale_rhs {
                self.value.sub(rhs.value.mul(scale))
            } else { 
                self.value.mul(scale).sub(rhs.value)
            }
        };
        let new_decimals = self.decimals.max(rhs.decimals);

        // Construct result
        Token {
            value: new_value,
            decimals: new_decimals,
        }
    }
}


impl PartialEq for Token {

    fn eq(&self, other: &Self) -> bool {

        // This checks whether we are going to scale other or self
        let scale_other: bool = self.decimals > other.decimals;

        // Calculate scale factor (will just be 0 if diff=0)
        let diff: u32 = self.decimals
            .max(other.decimals)
            .sub(self.decimals.min(other.decimals)) as u32;
        let scale = 10_u64.pow(diff);

        if scale_other {
            other.value.mul(scale) == self.value
        } else { 
            self.value.mul(scale) == other.value
        }
        
    }
}



#[test]
fn test_add_same_value_diff_decimals() {

    // 1 USD = 1_000_000 microUSD
    let token_1 = Token {
        value: 1_000_000,
        decimals: 6
    };

    // 1 USD = 1_000 milliUSD
    let token_2 = Token {
        value: 1_000,
        decimals: 3,
    };

    let sum_of_value = token_1 + token_2;

    // Check that it is equal when Token uses same precision as result
    // i.e. 2_000_000 microUSD = 2USD
    assert_eq!(
        sum_of_value,
        Token {
            value: 2_000_000,
            decimals: 6
        }
    );

    // But also when the token represents the same underlying value 
    // with a different precision.
    // i.e. 2_000_000 microUSD is still 2USD
    assert_eq!(
        sum_of_value,
        Token {
            value: 2,
            decimals: 0
        }
    )
}

#[test]
fn test_sub_diff_value_diff_decimals() {

    // 2 USD = 2_000_000 microUSD
    let token_1 = Token {
        value: 2_000_000,
        decimals: 6
    };

    // 1 USD = 1_000 milliUSD
    let token_2 = Token {
        value: 1_000,
        decimals: 3,
    };

    let sub_value = token_1 - token_2;

    // Check that it is equal when Token uses same precision as result
    // i.e. 1_000_000 microUSD = 1USD
    assert_eq!(
        sub_value,
        Token {
            value: 1_000_000,
            decimals: 6
        }
    );

    // But also when the token represents the same underlying value 
    // with a different precision.
    // i.e. 1_000_000 microUSD is still 1USD
    assert_eq!(
        sub_value,
        Token {
            value: 1,
            decimals: 0
        }
    )
}