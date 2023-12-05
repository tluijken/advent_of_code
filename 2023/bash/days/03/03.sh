#!/bin/bash
declare -A matrix
declare -A gears
l=0;
c=0;
sum=0
gearsTotal=0
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

is_adjacent_to_asterix() {
    local i=$1
    local j=$2
    
    if [[ ${matrix[$((i-1)),$((j-1))]} == "*" ]]; then
         echo "$((i-1)),$((j-1))"
    elif [[ ${matrix[$((i-1)),$j]} == "*" ]]; then
        echo "$((i-1)),$j"
    elif [[ ${matrix[$((i-1)),$((j+1))]} == "*" ]]; then
        echo "$((i-1)),$((j+1))"
    elif [[ ${matrix[$i,$((j-1))]} == "*" ]]; then
        echo "$i,$((j-1))"
    elif [[ ${matrix[$i,$((j+1))]} == "*" ]]; then
        echo "$i,$((j+1))"
    elif [[ ${matrix[$((i+1)),$((j-1))]} == "*" ]]; then
        echo "$((i+1)),$((j-1))"
    elif [[ ${matrix[$((i+1)),$j]} == "*" ]]; then
        echo "$((i+1)),$j"
    elif [[ ${matrix[$((i+1)),$((j+1))]} == "*" ]]; then
        echo "$((i+1)),$((j+1))"
    else
        echo 0
    fi
}


for (( i=0; i<l; i++ )); do
    for (( j=0; j<c; j++ )); do
        if [[ ${matrix[$i,$j]} =~ ^[0-9]+$ ]]; then
            adjentToSymbol=false
            num=${matrix[$i,$j]}
            is_adjacent_to_symbol "$i" "$j" && adjentToSymbol=true
            gearindex=$(is_adjacent_to_asterix "$i" "$j")
            while [[ ${matrix[$i,$((j+1))]} =~ ^[0-9]+$ ]]; do
                num=$num${matrix[$i,$((j+1))]}
                ((j++))
                is_adjacent_to_symbol "$i" "$j" && adjentToSymbol=true
                # if the gear index is not set, set it
                if [[ $gearindex == 0 ]]; then
                    gearindex=$(is_adjacent_to_asterix "$i" "$j")
                fi
            done
            [[ $adjentToSymbol == true ]] && ((sum+=$num))
            if [[ $gearindex != 0 ]]; then
                if [[ ${gears[$gearindex]} == "" ]]; then
                    # keep track of this gear index and the number, for later reference if we find this same asterix index again
                    gears[$gearindex]=$num
                else
                    ((gearsTotal+=${gears[$gearindex]} * num))
                fi
            fi
        fi
    done
done
echo $sum
echo $gearsTotal
