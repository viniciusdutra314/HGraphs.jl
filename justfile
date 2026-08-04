default:
    @just --list

# Download Copenhagen Networks Study version 1.
dataset-copenhagen:
    #!/usr/bin/env bash
    set -euo pipefail
    destination="{{justfile_directory()}}/hgraphs/tests/datasets/raw_files/copenhagen"
    archive="$destination/copenhagen.zip"
    mkdir -p "$destination"
    curl --fail --location --retry 3 --output "$archive" \
        'https://api.figshare.com/v2/articles/7267433/versions/1/download'
    unzip -o "$archive" -d "$destination"
    rm "$archive"
