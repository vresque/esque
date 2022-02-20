#!/bin/bash
RUSTNUM=`find ./ -name "*.rs" -print0 | xargs -0 cat | wc -l | sed 's/\r$//'`
DOCNUM=`find ./ -name "*.md" -print0 | xargs -0 cat | wc -l | sed 's/\r$//' | printf -v int '%d\n' 2>/dev/null`
PYNUM=`find ./ -name "*.py" -print0 | xargs -0 cat | wc -l | sed 's/\r$//' | printf -v int '%d\n' 2>/dev/null`
printf -v int '%d\n' "$RUSTNUM"