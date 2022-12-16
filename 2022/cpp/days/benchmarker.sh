start=$(date +%s%N)
./03
end=$(date +%s%N)
echo "Elapsed time: $(($end-$start)) ns"

start=$(date +%s%N)
$2
end=$(date +%s%N)
echo "Elapsed time: $(($(($end-$start))/1000000)) ms"
