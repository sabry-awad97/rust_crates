# Polars crate

## Introduction to Polars Crate

- Polars is a powerful data manipulation and analysis library for Rust.
- It provides a DataFrame abstraction similar to popular data manipulation tools like Pandas in Python or DataFrames in R.
- Polars offers a wide range of functions for data transformation, filtering, aggregation, and visualization.
- With Polars, you can efficiently process and analyze large datasets in Rust.

## Installing and Importing Polars Crate

- To use Polars in your Rust project, add it as a dependency in your `Cargo.toml` file.
- Open the `Cargo.toml` file and add the following line under the `[dependencies]` section:

```toml
polars = "0.29.0"
```

- Save the file, and Cargo will automatically download and install the Polars crate when you build your project.
- To import Polars in your Rust code, add the following line at the top of your Rust file:

```rust
use polars::prelude::*;
```

## Creating a Polars DataFrame

- A DataFrame is a tabular data structure consisting of rows and columns.
- To create a new DataFrame, you can use the `DataFrame::new()` function:

```rs
fn main() -> Result<(), PolarsError> {
    let df = DataFrame::new(vec![Series::new_empty("column1", &DataType::Float32)])?;
    println!("{}", df);
    Ok(())
}
```

- In this example, we create an empty DataFrame with a single column named "column1".

## Loading Data into a DataFrame

- Polars provides various methods to load data into a DataFrame, such as from CSV files or in-memory collections.
- To load data from a CSV file, you can use the `CsvReader`:

```rs
let df = CsvReader::from_path("data.csv")?
    .infer_schema(None)
    .has_header(true)
    .finish()?;
```

- In this example, we load data from a CSV file called "data.csv" and infer the schema automatically.

## DataFrame Operations

### Selecting Columns

- To select specific columns from a DataFrame, you can use the `select` method:

  ```rs
  let selected_df = df.select(&["column1", "column2"])?;
  ```

- This example selects columns "column1" and "column2" from the DataFrame `df`.

### Filtering Rows

- To filter rows based on certain conditions, you can use the `filter` method:

  ```rs
  let mask = df.column("column1")?.gt(10)?;
  let filtered_df = df.filter(&mask)?;
  ```

- This example filters rows where the value in "column1" is greater than 10.

### Adding and Modifying Columns

- To add or modify columns in a DataFrame, you can use the `with_column` method:

  ```rs
  let s = Series::new("new_column", [1, 2, 3, 4, 5]);
  let modified_df = df.with_column(s);
  ```

- This example adds a new column "new_column" to the df.

### Grouping and Aggregating Data

- Polars supports grouping and aggregating data using the `groupby` and `select` methods:

  ```rs
  let grouped_df = df
      .clone()
      .lazy()
      .groupby(["column1"])
      .agg(&[
          col("column2").alias("column2 count").count(),
          col("column3").alias("column3 mean").mean(),
      ])
      .collect()?;
  ```

- This example groups the DataFrame by "column1" and calculates the sum of "column2" and the mean of "column3" for each group.

### Sorting Data

- To sort a DataFrame based on one or more columns, you can use the `sort` method:

  ```rs
  let sorted_df = df.sort(&["column1", "column2"], vec![false, true])?;
  ```

- This example sorts the DataFrame first by "column1" in ascending order and then by "column2" in descending order.

### Joining DataFrame

- Polars allows you to join multiple DataFrames based on common columns using the `join` method:

  ```rs
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
  ```

- This example performs an inner join between `df1` and `df2` on "ID" from the left DataFrame and "ID" from the right DataFrame.

## Data Manipulation and Transformation

### Handling Missing Data

- Polars provides methods to handle missing data, such as `drop_nulls`, `fill_null`, or `null_count`.

```rs
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
```
