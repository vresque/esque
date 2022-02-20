#!/bin/bash
RUSTNUM=`find ./ -name "*.rs" -print0 | xargs -0 cat | wc -l | sed 's/\r$//'`
DOCNUM=`find ./ -name "*.md" -print0 | xargs -0 cat | wc -l | sed 's/\r$//'`
PYNUM=`find ./ -name "*.py" -print0 | xargs -0 cat | wc -l | sed 's/\r$//'`
ALLNUM=`expr $PYNUM + $RUSTNUM + $DOCNUM`
printf "There are %u lines of rust, $PYNUM lines of python and $DOCNUM lines of documentation." $RUSTNUM