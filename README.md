# rasterfakers

Generate fake rasters. Geotiffs or COGs.

## Usage

1. Generate a default GeoTIFF

```bash
cargo run -- --output fake.tif
```

2. Generate a GeoTIFF with custom parameters

```bash
cargo run -- \
    --output custom.tif \
    --width 512 \
    --height 512 \
    --bands 3 \
    --data-type u16 \
    --projection EPSG:3857 \
    --pixel-resolution "0.5,0.5" \
    --upper-left-corner "500000.0,2000000.0"
```

3. Using short options

```bash
cargo run -- \
    -o custom.tif \
    -w 512 \
    -e 512 \
    -b 3 \
    -t u16 \
    -p EPSG:3857 \
    -r "0.5,0.5" \
    -c "500000.0,2000000.0"
```

## Build CLI

```bash
cargo build --release
```
