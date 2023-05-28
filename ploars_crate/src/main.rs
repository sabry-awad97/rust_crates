use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = CsvReader::from_path("data.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let sorted_df = df.sort(&["Age"], false)?;

    println!("{}", sorted_df);
    Ok(())
}
