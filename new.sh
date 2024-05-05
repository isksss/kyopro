#!/bin/bash

cd `dirname $0`

new_contest=$1
cargo compete new $new_contest

cd atcoder/$new_contest