# db2md

db2md is a tool for converting Excel spreadsheets to Markdown files based on a YAML schema.

## Installation

1. Ensure you have Rust installed on your system. If not, you can install it from [official website](https://www.rust-lang.org/tools/install).

2. Clone this repository:
   ```
   git clone https://github.com/FIT-CNICE/db2md.git
   cd db2md
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. The executable will be available as `target/release/db2md`.

## Usage

Move `./target/release/db2md` to your `PATH` and simply run:

```
db2md
```

This will open a graphical user interface (GUI) for the db2md tool.

## Features and Responses

1. **Select XLSX File**
   - Click "Select XLSX" to choose an Excel file.
   - The selected file path will be displayed.
   - Click "Load" to process the file.
   - Response: "Loaded X rows of Y strings in SheetName"

2. **Select YAML Schema**
   - Click "Select YAML" to choose a YAML schema file.
   - The selected file path will be displayed.
   - Click "Load" to process the schema.
   - Response: 
     - "All fields found in selected yaml will be used to generate MD" (if successful)
     - "Find X fields but each row has Y columns, only first Z fields/columns will be used" (if mismatch)
     - "Invalid fields in Yaml [field1, field2, ...]" (if fields not found in Excel)

3. **Header Selection**
   - Choose whether the Excel file has a header row.

4. **File Prefix and Output Directory**
   - Set the prefix for generated Markdown files.
   - Set the output directory for generated files.

5. **Convert**
   - Click "Convert" to start the conversion process.
   - A progress bar will show the conversion progress.
   - Response:
     - "Fail to write rows: [row1, row2, ...]" (if any rows fail to convert)

## Prepare Your Yaml
- Each field may or may not have a data type("text", "date", "number",etc)
- Fields with data types correspond to columns in excel sheet
- For field name made of multiple words, you can use a format of "word1-word2-word3"
- Data types in your yaml does not need to be accurate, as the tool is intelligent enough to detect data types from metadata in excel

### A Yaml Example
```yaml
organization:
  sbu: text
  engineer: text
date: date
customer:
  department:
    product: text
    price: text
```
The yaml shown above will convert a row of 5 values into the following markdown:
```markdown
# Organization

## sub

text in column 1

## engineer

text in column 2

# date
text in column 3

# customer

## department

### product
text in column 4

### price
text in column 5
```
## Support

For issues or feature requests, please open an issue on the GitHub repository.

