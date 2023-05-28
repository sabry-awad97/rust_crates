use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = CsvReader::from_path("data.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let selected_df = df.select(&["Name", "Age"])?;
    println!("{}", selected_df);
    Ok(())
}
