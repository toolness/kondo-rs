This is a very simple, badly written Rust program to help you
Kondo your hard drive.

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
| 2502       | C:\Program Files\NVIDIA Corporation                | 
| 2135       | C:\Program Files\NVIDIA Corporation\Installer2     | 
| 1861       | C:\Program Files\Docker                            | 
| 1861       | C:\Program Files\Docker\Docker                     | 
| 1839       | C:\Program Files\Docker\Docker\resources           | 
| 1720       | C:\Program Files\QGIS 2.18                         | 

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

This is public domain.
