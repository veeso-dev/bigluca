#!/usr/bin/env bash

if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
  echo "Usage: <objects_dir> <start_n> <end_n>"
  exit 255
fi

objects_dir=$1
start_n=$2
end_n=$3

counter=$start_n

while [ $counter -le $end_n ]; do
  artifact_id=$(printf "%04d" $counter)
  # get json
  json_file=$(ls $objects_dir/$artifact_id-*.json | awk '{print $1}')
  echo "deleting cid $json_file"
  node delete.js "$json_file"

  let counter=counter+1
done
