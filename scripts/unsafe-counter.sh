unsafes=`grep -Rnw --include=\*.rs -e "unsafe" | wc -l`
rustlines=`find ./ -name "*.rs" -print0 | xargs -0 cat  | wc -l`
ratio=`echo "scale=5 ; $unsafes / $rustlines" | bc`
printf "A total of %d occurences have been found (%d LOC, %f Percent)\n" $unsafes $rustlines $ratio