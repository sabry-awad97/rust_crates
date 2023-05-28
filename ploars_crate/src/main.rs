use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df1 = df!(
        "ID" => &[1, 2, 3, 4],
        "Name" => &["John", "Emma", "Adam", "Emily"],
        "Department" => &["Digital Marketing", "Human Resources", "Finance", "Operations"]
    )?;

    let df2 = df!(
        "ID" => &[1, 2, 3, 5],
        "Salary" => &[5000, 4500, 6000, 5500]
    )?;

    let joined_df: DataFrame = df1.join(&df2, ["ID"], ["ID"], JoinType::Inner, None)?;

    println!("{}", joined_df);
    Ok(())
}
