use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = DataFrame::new(vec![Series::new_empty("column1", &DataType::Float32)])?;
    println!("{}", df);
    Ok(())
}
