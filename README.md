# mergedog - *the doc merger*

## Does the following
- Iterate through specified directory recursively
- Collect all `.pdf` files
- Merge them
- Save to `merged.pdf`

## `mergedog -h`
```
Merge PDF's in specified directory

Usage: mergedog [OPTIONS] [INPATH] [OUTPATH]

Arguments:
  [INPATH]   The path to be searched [default: .]
  [OUTPATH]  The path of the output file [default: merged.pdf]

Options:
  -a, --anno     Annotate file names to corner of first slides
  -q, --quiet    Quiet
  -h, --help     Print help
  -V, --version  Print version
```

## Install
`cargo install --git https://github.com/ManuelPardoPose/mergedog.git`
