use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let mut df = CsvReader::from_path("missing.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    println!("{}", df);

    // Check if there are null values
    println!("Null values: \n{}", df.null_count());

    // Drops rows with any null value
    let df_without_nulls = df.drop_nulls::<String>(None)?;
    println!("DataFrame after dropping nulls:\n{:?}", df_without_nulls);

    // Fill null values with a default value
    let selected_columns = df.column("Age")?;
    let column_filled = selected_columns.fill_null(FillNullStrategy::Zero)?;
    let df_filled = df.with_column(column_filled)?;
    println!("DataFrame after filling nulls:\n{:?}", df_filled);
    Ok(())
}
