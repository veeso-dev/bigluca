#!/usr/bin/env bash

if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] || [ -z "$4" ]; then
  echo "Usage: <artifacts_dir> <output_dir> <start_n> <end_n>"
  exit 255
fi

artifacts_dir=$1
output_dir=$2
start_n=$3
end_n=$4

counter=$start_n

set -e
while [ $counter -le $end_n ]; do
  artifact_id=$(printf "%04d" $counter)
  # get json
  json_file=$(ls $artifacts_dir/$artifact_id-*.json | awk '{print $1}')
  png_file=$(ls $artifacts_dir/$artifact_id-*.png | awk '{print $1}')
  echo "uploading file $json_file;$png_file"
  node index.js "$json_file" "$png_file" "$output_dir"

  let counter=counter+1
done
