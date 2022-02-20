#!/bin/bash
RUSTNUM=`find ./ -name "*.rs" -print0 | xargs -0 cat | wc -l | sed 's/\r$//'`
DOCNUM=`find ./ -name "*.md" -print0 | xargs -0 cat | wc -l | sed 's/\r$//'`
PYNUM=`find ./ -name "*.py" -print0 | xargs -0 cat | wc -l | sed 's/\r$//' `
printf "%u" $RUSTNUM
echo " lines of rust."
printf "%u" $PYNUM
echo " lines of python."
printf "%u" $DOCNUM
echo " lines of markdown."
