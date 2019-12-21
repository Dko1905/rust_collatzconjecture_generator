# Collatz conjecture generator
Really fast generator written in rust. It uses bitshifting to optimize calculations. 

## RCG (Rust Collatz conjecture Generator)
Fast and saves the output to a `.csv` file.
### Example
The syntax is `./rcg {result filename} {upper limit to generate}`.
```bash
$ ./rcg "result.csv" 1000
```

## Plotting
There is also a Rscript in the plottting directory, it plots the result from the `.csv` file.
### Example
The syntax is `Rscript {input filename} {output filename (.png only)}`.
```bash
$ Rscript lenght.r "result.csv" "result.png"
null device 
          1
```
It's normal that it says `null device 1`.

## Docker
There is no docker, since I don't really know how to make one that takes user arguments.

## Licence
[MIT]([www.google.com](https://opensource.org/licenses/MIT))