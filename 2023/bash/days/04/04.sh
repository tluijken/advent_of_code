#!/bin/bash
sum=0
declare -A m
originals=0;
while read -r line; do
    ((originals++))
    cid=$originals
    # get the number of copies of this card out of the m array
    copies=${m[$cid]}
    [[ $copies == "" ]] && copies=0
    n=$(sed "s/Card $cid: //g" <<< "$line")
    IFS='|' read -ra nsplit <<< "$n"
    IFS=' ' read -ra wn <<< "${nsplit[0]}"
    IFS=' ' read -ra mn <<< "${nsplit[1]}"
    for ((c=0; c <= $copies; c++)); do
        matchCount=0
        for i in "${wn[@]}"; do
            for j in "${mn[@]}"; do
                [[ $i == $j ]] && ((matchCount++))
            done
        done
        [[ $matchCount == 0 ]] && continue
        [[ $c == 0 ]] && sum=$((sum + (2 ** (matchCount - 1))))
        for ((i = cid + 1; i <= matchCount + cid; i++)); do
            m[$i]=$((m[$i] + 1))
        done
    done
done < $1
echo $sum

decksum=$originals
for i in "${!m[@]}"; do
    decksum=$((decksum + m[$i]))
done
echo $decksum
