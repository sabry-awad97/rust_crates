use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = CsvReader::from_path("data.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let grouped_df = df
        .clone()
        .lazy()
        .groupby(["Gender"])
        .agg(&[
            col("Age").alias("Age Count").count(),
            col("Age").alias("Age Mean").mean(),
        ])
        .collect()?;

    println!("{}", grouped_df);
    Ok(())
}
