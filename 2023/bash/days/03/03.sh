#!/bin/bash
declare -A m; declare -A g
l=0;c=0;sum=0;gearsTotal=0;symbols='[^.0-9]';asterix='[*]'

positions=(-1,-1 -1,0 -1,1 0,-1 0,1 1,-1 1,0 1,1)

while read -r line; do
    for (( i=0; i<${#line}; i++ )); do
        m[$l,$i]=${line:$i:1}
        [[ $i -gt $c ]] && c=$i
    done
    ((l++))
done < $1

check_adjacent() {
    local i=$1;local j=$2;local pattern=$3

    for (( k=0; k<${#positions[@]}; k++ )); do
        IFS=',' read -ra pos <<< "${positions[$k]}"
        if [[ ${m[$((i+pos[0])),$((j+pos[1]))]} =~ $pattern ]]; then
            echo $((i+pos[0])),$((j+pos[1]))
            return 0
        fi
    done
    echo 0
}

for (( i=0; i<l; i++ )); do
    for (( j=0; j<c; j++ )); do
        if [[ ${m[$i,$j]} =~ ^[0-9]+$ ]]; then
            num=${m[$i,$j]}
            si=$(check_adjacent "$i" "$j" "$symbols")
            gi=$(check_adjacent "$i" "$j" "$asterix")
            while [[ ${m[$i,$((j+1))]} =~ ^[0-9]+$ ]]; do
                num=$num${m[$i,$((j+1))]}
                ((j++))
                [[ $si == 0 ]] && si=$(check_adjacent "$i" "$j" "$symbols")
                [[ $gi == 0 ]] && gi=$(check_adjacent "$i" "$j" "$asterix")
            done
            [[ $si != 0 ]] && ((sum+=$num))
            [[ $gi == 0 ]] && continue
            if [[ ${g[$gi]} == "" ]]; then
                g[$gi]=$num
            else
                ((gearsTotal+=${g[$gi]} * num))
            fi
        fi
    done
done
echo $sum
echo $gearsTotal
