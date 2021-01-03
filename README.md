# nbconvert

`nbconvert` is a very basic tool for extracting the code from a Jupyter
notebook file. When editing notebooks, content regularly changes such as cell
run counts and image outputs.

Exporting within Jupyter itself is also a little messy, leaving a lot of
whitespace and the cell counts inside the file it produces. `nbconvert` instead
just produces the code itself, as if you had written it as a standalone file
making it easier to work with in Git.

## Installation

`nbconvert` can be installed by cloning the repository and using `cargo` to
build and install a release binary:

```bash
git clone git@github.com:alexander-jackson/nbconvert.git
cd nbconvert/
cargo install --path .
```

## Usage

To convert a Jupyter notebook to a Python file, invoke as follows:

```bash
nbconvert -i notebook.ipynb -o output.py
```
