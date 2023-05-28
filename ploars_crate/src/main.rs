use polars::prelude::*;
use polars_ops::pivot::{pivot, PivotAgg};

fn main() -> Result<(), PolarsError> {
    let df = df!(
        "Region" => &["East", "East", "East", "West", "West", "West"],
        "Year" => &[2019, 2019, 2020, 2019, 2019, 2020],
        "Quarter" => &["Q1", "Q2", "Q1", "Q1", "Q2", "Q1"],
        "Sales" => &[100, 150, 200, 120, 180, 220],
    )?;

    let pivoted_df = pivot(
        &df,
        ["Sales"],
        ["Quarter"],
        ["Year"],
        true,
        Some(PivotAgg::Sum),
        None,
    )?;

    println!("{}", df);
    println!("{}", pivoted_df);
    Ok(())
}
