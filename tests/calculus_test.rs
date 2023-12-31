use std::str::FromStr;

use baggins::{discount, tax, Calculator};
use bigdecimal::BigDecimal;

#[test]
fn test_baggins_compute() {
    let mut c = baggins::DetailCalculator::new();

    let err = c.add_discount_from_str("10.0", discount::Type::Percentual);
    assert!(err.is_none(), "error adding percentual discount {:?}", err);

    let err = c.add_discount_from_str("1.0", discount::Type::AmountUnit);
    assert!(err.is_none(), "error adding amount unit discount {:?}", err);

    // let err: Option<discount::DiscountError> = c.add_str_discount("2.0", discount::Type::AmountLine);
    // assert!(err.is_none(), "error adding amount line discount");

    let err = c.add_tax_from_str(
        "16.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::Percentual,
    );
    assert!(err.is_none(), "error adding percentual 16% tax {:?}", err);

    let err = c.add_tax_from_str(
        "1.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::AmountUnit,
    );
    assert!(
        err.is_none(),
        "error adding percentual 1 amount unit tax {:?}",
        err
    );

    let r = c.compute(
        BigDecimal::from_str("100.0").unwrap(),
        BigDecimal::from_str("2.0").unwrap(),
        16,
    );

    match r {
        Ok(calc) => {
            println!("calc: {}", calc);
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}

#[test]
fn test_baggins_compute_from_brute() {
    let mut c = baggins::DetailCalculator::new();

    let err = c.add_discount_from_str("10.0", discount::Type::Percentual);
    assert!(err.is_none(), "error adding percentual discount {:?}", err);

    let err = c.add_discount_from_str("1.0", discount::Type::AmountUnit);
    assert!(err.is_none(), "error adding amount unit discount {:?}", err);

    // let err: Option<discount::DiscountError> = c.add_str_discount("2.0", discount::Type::AmountLine);
    // assert!(err.is_none(), "error adding amount line discount");

    let err = c.add_tax_from_str(
        "16.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::Percentual,
    );
    assert!(err.is_none(), "error adding percentual 16% tax {:?}", err);

    let err = c.add_tax_from_str(
        "1.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::AmountUnit,
    );
    assert!(
        err.is_none(),
        "error adding percentual 1 amount unit tax {:?}",
        err
    );

    let r = c.compute_from_brute(
        BigDecimal::from_str("208.4800").unwrap(),
        BigDecimal::from_str("2.0").unwrap(),
        16,
    );

    match r {
        Ok(calc) => {
            println!("calc: {}", calc);
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}

#[test]
fn test_baggins_compute_with_line_discount() {
    let mut c = baggins::DetailCalculator::new();

    let err = c.add_discount_from_str("10.0", discount::Type::Percentual);
    assert!(err.is_none(), "error adding percentual discount {:?}", err);

    let err = c.add_discount_from_str("1.0", discount::Type::AmountUnit);
    assert!(err.is_none(), "error adding amount unit discount {:?}", err);

    let err: Option<discount::DiscountError> =
        c.add_discount_from_str("2.0", discount::Type::AmountLine);
    assert!(err.is_none(), "error adding amount line discount {:?}", err);

    let err = c.add_tax_from_str(
        "16.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::Percentual,
    );
    assert!(err.is_none(), "error adding percentual 16% tax {:?}", err);

    let err = c.add_tax_from_str(
        "1.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::AmountUnit,
    );
    assert!(
        err.is_none(),
        "error adding percentual 1 amount unit tax {:?}",
        err
    );

    let r = c.compute(
        BigDecimal::from_str("100.0").unwrap(),
        BigDecimal::from_str("2.0").unwrap(),
        16,
    );

    match r {
        Ok(calc) => {
            println!("calc: {}", calc);
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}

#[test]
fn test_baggins_compute_with_line_discount_from_brute() {
    let mut c = baggins::DetailCalculator::new();

    let err = c.add_discount_from_str("10.0", discount::Type::Percentual);
    assert!(err.is_none(), "error adding percentual discount {:?}", err);

    let err = c.add_discount_from_str("1.0", discount::Type::AmountUnit);
    assert!(err.is_none(), "error adding amount unit discount {:?}", err);

    let err: Option<discount::DiscountError> =
        c.add_discount_from_str("2.0", discount::Type::AmountLine);
    assert!(err.is_none(), "error adding amount line discount {:?}", err);

    let err: Option<tax::TaxError> = c.add_tax_from_str(
        "16.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::Percentual,
    );
    assert!(err.is_none(), "error adding percentual 16% tax {:?}", err);

    let err = c.add_tax_from_str(
        "1.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::AmountUnit,
    );
    assert!(
        err.is_none(),
        "error adding percentual 1 amount unit tax {:?}",
        err
    );

    let r = c.compute_from_brute(
        BigDecimal::from_str("206.1600").unwrap(),
        BigDecimal::from_str("2.0").unwrap(),
        16,
    );

    match r {
        Ok(calc) => {
            println!("calc: {}", calc);
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}
