#!/bin/bash

# Basic usage
rasterfakers -o basic.tiff

# Custom dimensions and data type
rasterfakers -o high_res.tiff -w 1024 -e 1024 -t u16

# Multiple bands with sine wave pattern
rasterfakers -o rgb_sine.tiff -b 3 -n sine -t u8

# Custom projection and resolution
rasterfakers --output=projected.tiff \
    --projection="EPSG:3857" \
    --pixel-resolution="0.1,0.1" \
    --upper-left-corner="-180.0,90.0"

# Noise pattern with custom resolution
rasterfakers -o noise.tiff \
    -n noise \
    -t f32 \
    -r "0.01,0.01"

# Cloud Optimized GeoTIFF
rasterfakers -o cog_basic.tiff --cloud-optimized

# Cloud Optimized GeoTIFF with custom settings
rasterfakers -o cog_custom.tiff \
    -w 512 \
    -e 512 \
    -b 3 \
    -t f32 \
    -n sine \
    -p "EPSG:4326" \
    -r "0.01,0.01" \
    --cloud-optimized

echo "All example GeoTIFFs generated successfully!"
