#!/bin/bash

cd `dirname $0`

new_contest=$1
mondai=$2

cd atcoder/$new_contest

cargo compete submit $mondai