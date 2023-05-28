use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = CsvReader::from_path("data.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let mask = df.column("Age")?.gt(40)?;
    let filtered_df = df.filter(&mask)?;
    println!("{}", filtered_df);
    Ok(())
}
