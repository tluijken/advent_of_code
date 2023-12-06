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
    matchcount=0
    for i in "${wn[@]}"; do
        for j in "${mn[@]}"; do
            [[ $i == $j ]] && ((matchcount++))
        done
    done
    [[ $matchcount == 0 ]] && continue
    sum=$((sum + (2 ** (matchcount - 1))))
    for ((i = cid + 1; i <= matchcount + cid; i++)); do
        m[$i]=$((m[$i] + 1 + copies))
    done
done < $1
echo $sum

decksum=$originals
for i in "${!m[@]}"; do
    decksum=$((decksum + m[$i]))
done
echo $decksum
