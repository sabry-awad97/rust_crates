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
          col("column2").count(),
          col("column3").mean(),
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

### Applying Functions to Columns

#### Applying a Closure

```rs
let mut df = DataFrame::new(vec![
    Series::new("column1", &[1, 2, 3]),
    Series::new("column2", &[2, 4, 6]),
])?;

df.apply("column1", |value| value * 2)?;
```

#### Applying a Predefined Function

```rs
fn square(x: &Series) -> Series {
    x * x
}

let mut df = DataFrame::new(vec![
    Series::new("column1", &[1, 2, 3]),
    Series::new("column2", &[4, 5, 6]),
])?;

df.apply("column1", square)?;
```

### Reshaping DataFrames

- Polars offers functions like pivot and melt for reshaping DataFrames.

#### Pivot Operation

- The pivot operation in Polars allows you to reshape a DataFrame by converting unique values from one column into multiple columns.
- It is particularly useful when you want to transform long-format data into wide-format data.

- Suppose you have the following DataFrame `df` representing sales data:
  | Region | Year | Quarter | Sales |
  | ------ | ---- | ------- | ----- |
  | East | 2019 | Q1 | 100 |
  | East | 2019 | Q2 | 150 |
  | East | 2020 | Q1 | 200 |
  | West | 2019 | Q1 | 120 |
  | West | 2019 | Q2 | 180 |
  | West | 2020 | Q1 | 220 |
  To pivot the DataFrame based on the "Quarter" column, with "Year" as the new column names and "Sales" as the new column values, you can use the pivot operation:

  ```rs
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
  ```

  The resulting pivoted DataFrame `pivoted_df` will look like this:
  | Quarter | 2019 | 2020 |
  | ------- | ---- | ---- |
  | Q1 | 220 | 420 |
  | Q2 | 330 | null |

  In this example, the unique values from the "Quarter" column become the column names in the pivoted DataFrame, with corresponding values from the "Sales" column.

#### Melt Operation

- The melt operation in Polars is the inverse of the pivot operation. It allows you to transform wide-format data into long-format data.
- With the melt operation, you can gather multiple columns into a single column, creating a key-value pair representation.
- The melt operation requires two parameters: the id_vars, which are the columns to keep as identifiers, and the value_vars, which are the columns to melt.
- Melt Operation Example: Suppose you have the following DataFrame `df` representing customer information:
  | Customer | Gender | Age | Income |
  | -------- | ------ | --- | ------ |
  | Alice | Female | 25 | 50000 |
  | Bob | Male | 30 | 60000 |
  | Charlie | Male | 35 | 70000 |
  To melt the DataFrame by keeping the "Customer" column as the identifier and melting the "Gender", "Age", and "Income" columns, you can use the melt operation:

  ```rs
  let df = df!(
      "Customer" => ["Alice", "Bob", "Charlie"],
      "Gender" => ["Female", "Male", "Male"],
      "Age" => [25, 30, 35],
      "Income" => [50000, 60000, 70000]
  )?;

  let melted_df = df.melt(&["Customer"], &["Gender", "Age", "Income"])?;

  println!("{}", df);
  println!("{}", melted_df);
  ```

The resulting melted DataFrame `melted_df` will look like this:
| Customer | variable | value |
| -------- | -------- | ------ |
| Alice | Gender | Female |
| Bob | Gender | Male |
| Charlie | Gender | Male |
| Alice | Age | 25 |
| Bob | Age | 30 |
| Charlie | Age | 35 |
| Alice | Income | 50000 |
| Bob | Income | 60000 |
| Charlie | Income | 70000 |

In this example, the columns "Gender", "Age", and "Income" are melted into a single column named "variable", and their corresponding values are captured in the "value" column.

### Data Type Conversion

Data Type Conversion refers to the process of changing the data type of a column in a DataFrame. Polars provides methods to convert the data types of DataFrame columns, allowing you to transform the data to a format that is suitable for your analysis or computations.

Converting data types can be useful when you need to perform calculations or operations that require specific data types, or when you want to ensure consistency and compatibility between columns.

To perform data type conversion in Polars, you can use the `cast` method. The `cast` method allows you to specify the target data type for a column. Here's an example:

```rs
let converted_df = df
    .lazy()
    .select(&[col("column2").cast(DataType::Int32)])
    .collect()?;
```

It's important to note that data type conversion may result in data loss or unexpected behavior if the conversion is not compatible or if the data contains values that cannot be converted. Therefore, it's recommended to handle data type conversion with caution and ensure that the conversion is appropriate for your data and analysis.
