This is a very simple, badly written Rust program to help you
[Kondo][] your hard drive.

Given a root directory, it simply finds all files and directories
within that are at least 50mb in size, and outputs them
from largest to smallest in `kondo.csv`.

Here's an example of `kondo.csv`:

| size in mb | path                                               | 
|------------|----------------------------------------------------| 
| 15482      | C:\Program Files                                   | 
| 3354       | C:\Program Files\Unity                             | 
| 3235       | C:\Program Files\Unity\Editor                      | 
| 2712       | C:\Program Files\Unity\Editor\Data                 | 
| 2686       | C:\Program Files\NVIDIA GPU Computing Toolkit      | 
| 2686       | C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA | 

[Kondo]: https://en.wikipedia.org/wiki/Marie_Kondo

## Quick start

First, install it:

```
cargo install --path . --force
```

Then run it:

```
kondo C:\
```

## License

[CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/) (public domain)
