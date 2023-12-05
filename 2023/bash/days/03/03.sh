#!/bin/bash
declare -A matrix
l=0;
c=0;
while read -r line; do
    for (( i=0; i<${#line}; i++ )); do
        char=${line:$i:1}
        matrix[$l,$i]=$char
        [[ $i -gt $c ]] && c=$i
    done
    ((l++))
done < input

sum=0
for (( i=0; i<l; i++ )); do
    for (( j=0; j<c; j++ )); do
        if [[ ${matrix[$i,$j]} =~ ^[0-9]+$ ]]; then
            adjentToSymbol=false
            num=${matrix[$i,$j]}
            # check if number has neighbours that are any symbol (non alpha, non numeric) except a dot, even diagonally
            if [[ ${matrix[$((i-1)),$((j-1))]} =~ [^.0-9] ]] || 
                [[ ${matrix[$((i-1)),$j]} =~ [^.0-9] ]] ||
                [[ ${matrix[$((i-1)),$((j+1))]} =~ [^.0-9] ]] ||
                [[ ${matrix[$i,$((j-1))]} =~ [^.0-9] ]] ||
                [[ ${matrix[$i,$((j+1))]} =~ [^.0-9] ]] ||
                [[ ${matrix[$((i+1)),$((j-1))]} =~ [^.0-9] ]] ||
                [[ ${matrix[$((i+1)),$j]} =~ [^.0-9] ]] ||
                [[ ${matrix[$((i+1)),$((j+1))]} =~ [^.0-9] ]]; then
                adjentToSymbol=true
            fi

            while [[ ${matrix[$i,$((j+1))]} =~ ^[0-9]+$ ]]; do
                num=$num${matrix[$i,$((j+1))]}
                ((j++))
                if [[ ${matrix[$((i-1)),$((j-1))]} =~ [^.0-9] ]] || 
                    [[ ${matrix[$((i-1)),$j]} =~ [^.0-9] ]] ||
                    [[ ${matrix[$((i-1)),$((j+1))]} =~ [^.0-9] ]] ||
                    [[ ${matrix[$i,$((j-1))]} =~ [^.0-9] ]] ||
                    [[ ${matrix[$i,$((j+1))]} =~ [^.0-9] ]] ||
                    [[ ${matrix[$((i+1)),$((j-1))]} =~ [^.0-9] ]] ||
                    [[ ${matrix[$((i+1)),$j]} =~ [^.0-9] ]] ||
                    [[ ${matrix[$((i+1)),$((j+1))]} =~ [^.0-9] ]] then
                    adjentToSymbol=true
                fi
            done
            if [[ $adjentToSymbol == true ]]; then
                ((sum+=$num))
            fi
        fi
    done
done
echo $sum
