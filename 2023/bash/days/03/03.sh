#!/bin/bash
declare -A matrix
l=0;
c=0;
symbol_regex='[^.0-9]'
while read -r line; do
    for (( i=0; i<${#line}; i++ )); do
        char=${line:$i:1}
        matrix[$l,$i]=$char
        [[ $i -gt $c ]] && c=$i
    done
    ((l++))
done < input

is_adjacent_to_symbol() {
    local i=$1
    local j=$2
    if [[ ${matrix[$((i-1)),$((j-1))]} =~ $symbol_regex ]] || 
        [[ ${matrix[$((i-1)),$j]} =~ $symbol_regex ]] ||
        [[ ${matrix[$((i-1)),$((j+1))]} =~ $symbol_regex ]] ||
        [[ ${matrix[$i,$((j-1))]} =~ $symbol_regex ]] ||
        [[ ${matrix[$i,$((j+1))]} =~ $symbol_regex ]] ||
        [[ ${matrix[$((i+1)),$((j-1))]} =~ $symbol_regex ]] ||
        [[ ${matrix[$((i+1)),$j]} =~ $symbol_regex ]] ||
        [[ ${matrix[$((i+1)),$((j+1))]} =~ $symbol_regex ]]; then
        return 0
    else
        return 1
    fi
}

sum=0
for (( i=0; i<l; i++ )); do
    for (( j=0; j<c; j++ )); do
        if [[ ${matrix[$i,$j]} =~ ^[0-9]+$ ]]; then
            adjentToSymbol=false
            num=${matrix[$i,$j]}
            is_adjacent_to_symbol "$i" "$j" && adjentToSymbol=true
            while [[ ${matrix[$i,$((j+1))]} =~ ^[0-9]+$ ]]; do
                num=$num${matrix[$i,$((j+1))]}
                ((j++))
                is_adjacent_to_symbol "$i" "$j" && adjentToSymbol=true
            done
            [[ $adjentToSymbol == true ]] && ((sum+=$num))
        fi
    done
done
echo $sum
