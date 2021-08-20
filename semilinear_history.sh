#!/bin/sh

set -e

git fetch origin master

local_hash=$(git rev-parse HEAD)
echo $local_hash

hash1=$(git show-ref -s origin/master)
echo $hash1

hash2=$(git merge-base $hash1 $local_hash)
echo $hash2

if [ "${hash1}" = "${hash2}" ]
then
  echo "Branch is rebased on origin/master. Let's proceed."
else
  echo "Branch needs to be rebased on origin/master!"
  exit 1
fi
