# Rusty TF-IDF

This library implements [TF-IDF](https://en.wikipedia.org/wiki/Tf%E2%80%93idf) in Rust. 

## Implementation

I have used following functions for TF and IDF parts:

**TF**:  

![tf](https://latex.codecogs.com/gif.latex?1&plus;%5Clog%20f_%7B%7Bt%2Cd%7D%7D)

**IDF**:  

![idf](https://latex.codecogs.com/gif.latex?%5Clog%281&plus;%7B%5Cfrac%20%7BN%7D%7Bn_%7Bt%7D%7D%7D%29)

## Dependencies

Rust > 1.x

## Build

```
cargo build
```

## Test

```
cargo test
```

## Build documentation

Run follwoing command:

```
cargo doc
```

## Author

Afshin Mehrabani - afshin.meh@gmail.com

## License
MIT
