use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = df!(
        "Customer" => ["Alice", "Bob", "Charlie"],
        "Gender" => ["Female", "Male", "Male"],
        "Age" => [25, 30, 35],
        "Income" => [50000, 60000, 70000]
    )?;

    let melted_df = df.melt(&["Customer"], &["Gender", "Age", "Income"])?;

    println!("{}", df);
    println!("{}", melted_df);
    Ok(())
}
