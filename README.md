# cli-doc-merger

## Does the following
- Iterate through specified directory recursively
- Standard directory is `.`
- Collect all `.pdf` files
- Merge them
- Save to `merged.pdf`

## `cli-doc-merger -h`
```
Merge PDF's in specified directory

Usage: mergepdfs [INPATH] [OUTPATH]

Arguments:
  [INPATH]   The path to be searched [default: .]
  [OUTPATH]  The path of the output file [default: merged.pdf]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Install
`cargo install --git https://github.com/ManuelPardoPose/cli-doc-merger.git`
