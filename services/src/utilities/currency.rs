use crate::primitives::{
    bigint::{FromPrimitive as _, Pow as _, ToPrimitive as _},
    Uint256,
};

pub fn calculate_amount(amount: &Uint256, decimals: i32) -> f64 {
    let exp = if decimals > 6 { decimals - 6 } else { 0 };
    let base = Uint256::from_u32(10u32).unwrap().pow(exp);

    let amount = amount / base;
    let amount = amount.to_f64().unwrap_or_default();
    let base = 10f64.powi(if decimals > 6 { 6 } else { decimals });

    amount / base
}
