# Build

We will require on a number of optional C dependencies, such as GEOS, PROJ, and GDAL. To simplify these dependencies, we use Pixi, a package management system for native dependencies with an easy-to-use lockfile.

## Install Pixi

Follow the instructions given [here](https://pixi.sh/latest/#installation).

Once installed

```bash
cd build
pixi install
pixi shell
```

Export the following environment variables at the root of the repository (Maybe - first try without it):

```bash
export GDAL_HOME="$(pwd)/build/.pixi/envs/default"
export DYLD_LIBRARY_PATH="$(pwd)/build/.pixi/envs/default/lib:$DYLD_LIBRARY_PATH"
export PKG_CONFIG_PATH="$(pwd)/build/.pixi/envs/default/lib/pkgconfig:$PKG_CONFIG_PATH"
export GDAL_VERSION=3.9.0
export GDAL_INCLUDE_DIR="$(pwd)/build/.pixi/envs/default/include"
export GDAL_LIB_DIR="$(pwd)/build/.pixi/envs/default/lib"
```
