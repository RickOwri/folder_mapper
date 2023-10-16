# File Mapping Tool

The File Mapping Tool is a Rust-based command-line application designed to create a SQLite database that stores mappings between filenames and their corresponding file paths in a specified folder.

## Prerequisites

Before using this tool, ensure that you have Rust and its package manager, Cargo, installed on your system.

## Usage

To use the File Mapping Tool, follow these steps:

1. Clone this repository to your local machine.

2. Build the tool by running the following command in the project directory:

   ```shell
   cargo build --release
   ```

3. Run the tool with the path to the folder you want to map as the only command-line argument:

   ```shell
   ./target/release/file-mapping-tool <folder_path>
   ```

   Replace `<folder_path>` with the path to the folder you want to create a mapping database for.

## Example

Here's an example usage of the tool:

```shell
./target/release/file-mapping-tool /path/to/your/folder
```

## Output

The tool will create or connect to an SQLite database in the specified folder and store mappings between filenames and their corresponding file paths. The database will be named `0_SQL_allfiles.db`.
