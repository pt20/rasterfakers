#!/bin/bash

# Basic usage
rasterfakers -o basic.tiff

# Custom dimensions and data type
rasterfakers -o high_res.tiff -w 1024 -e 1024 -t u16

# Multiple bands with sine wave pattern
rasterfakers -o rgb_sine.tiff -b 3 -n sine -t u8

# Custom projection and resolution
rasterfakers -o projected.tiff \
    -p "EPSG:3857" \
    -r "0.1,0.1" \
    -c "-180.0,90.0"

# Noise pattern with custom resolution
rasterfakers -o noise.tiff \
    -n noise \
    -t f32 \
    -r "0.01,0.01"

echo "All example GeoTIFFs generated successfully!"
