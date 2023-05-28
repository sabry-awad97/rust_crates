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
