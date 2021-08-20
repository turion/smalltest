#!/bin/sh

local_hash=$(git rev-parse HEAD)
hash1=$(git show-ref -s origin/master)
hash2=$(git merge-base origin/master $local_hash)

if [ "${hash1}" = "${hash2}" ]
then
  echo "Branch is rebased on origin/master. Let's proceed."
else
  echo "Branch needs to be rebased on origin/master!"
  exit 1
fi
