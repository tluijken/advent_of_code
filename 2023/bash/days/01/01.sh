#!/bin/bash
declare -A numbers
numbers=(["one"]=1 ["two"]=2 ["three"]=3 ["four"]=4 ["five"]=5 ["six"]=6 ["seven"]=7 ["eight"]=8 ["nine"]=9)
readarray -t lines < ./input

total=0
# get the first 100 lines
for line in "${lines[@]}"; do
    readarray -t num <<< $(echo "$line" | sed 's/one/one1one/g;s/two/two2two/g;s/three/three3three/g;s/four/four4four/g;s/five/five5five/g;s/six/six6six/g;s/seven/seven7seven/g;s/eight/eight8eight/g;s/nine/nine9nine/g' | grep -oP '(one|two|three|four|five|six|seven|eight|nine|[0-9])')
    [[ ${num[0]} =~ ^[0-9]+$ ]] && first=${num[0]} || first=${numbers[${num[0]}]}
    [[ ${num[-1]} =~ ^[0-9]+$ ]] && last=${num[-1]} || last=${numbers[${num[-1]}]}
    total=$((total + "$first$last"))
done

echo "$total"
