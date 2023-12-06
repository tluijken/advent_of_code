#!/bin/bash
sum=0
while read -r line; do
    cid=$(grep -oP 'Card \K[0-9]+' <<< "$line")
    n=$(sed "s/Card $cid: //g" <<< "$line")
    IFS='|' read -ra nsplit <<< "$n"
    IFS=' ' read -ra wn <<< "${nsplit[0]}"
    IFS=' ' read -ra mn <<< "${nsplit[1]}"
    score=0
    for i in "${wn[@]}"; do
        # check if word is in mn
        for j in "${mn[@]}"; do
            if [[ $i == $j ]]; then
                if [[ $score == 0 ]]; then
                    score=1
                else
                    score=$((score * 2))
                fi
            fi
        done
    done
    sum=$((sum + score))
done < $1
echo $sum
