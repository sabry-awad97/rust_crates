use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = DataFrame::new(vec![
        Series::new("column1", &[1, 2, 3]),
        Series::new("column2", &["4", "5", "6"]),
    ])?;
    let converted_df = df
        .lazy()
        .select(&[col("column2").cast(DataType::Int32)])
        .collect()?;
    println!("{}", converted_df);
    Ok(())
}
