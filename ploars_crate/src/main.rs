use polars::prelude::*;

fn square(x: &Series) -> Series {
    x * x
}

fn main() -> Result<(), PolarsError> {
    let mut df = DataFrame::new(vec![
        Series::new("column1", &[1, 2, 3]),
        Series::new("column2", &[4, 5, 6]),
    ])?;

    df.apply("column1", square)?;

    println!("{}", df);
    Ok(())
}
