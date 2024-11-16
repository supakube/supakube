#!/bin/bash 

# Remove a leading v if there is one
VERSION=$(echo $1 | sed 's/v//g')
sed -i "0,/version/{s/version.*$/version = \"$VERSION\"/}" ../../crates/supakube/Cargo.toml

# Update all the version number of the bionic operator
sed -i "/name = \"supakube\"/{n;s/.*/version = \"$VERSION\"/}" ../../Cargo.lock
