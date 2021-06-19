#!/bin/bash
mkdir $1
touch $1/$1.rs
echo "rustc $1.rs && ./$1" >> $1/build-run.sh
chmod +x $1/build-run.sh
cd $1
