#!/bin/bash
declare -A m
cid=0;sum=0
while read -r line; do
    ((cid++))
    copies=${m[$cid]:-0}
    n=$(sed "s/Card $cid: //g" <<< "$line")
    IFS='|' read -ra nsplit <<< "$n"
    IFS=' ' read -ra wn <<< "${nsplit[0]}"
    IFS=' ' read -ra mn <<< "${nsplit[1]}"
    intersection=($(comm -12 <(printf '%s\n' "${wn[@]}" | sort -u) <(printf '%s\n' "${mn[@]}" | sort -u)))
    matchcount=${#intersection[@]}  
    [[ $matchcount == 0 ]] && continue
    sum=$((sum + (2 ** (matchcount - 1))))
    for i in $(seq $((cid + 1)) $((matchcount + cid))); do ((m[$i] += 1 + copies)); done
done < $1
echo $sum

for i in "${!m[@]}"; do cid=$((cid + m[$i]));done
echo $cid
