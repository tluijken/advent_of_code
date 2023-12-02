#!/bin/bash
#
gameIdTotals=0
powers=0
declare -A thresholds=([red]=12 [blue]=14 [green]=13)
while read -r line; do

    gameId=$(grep -oP 'Game \K[0-9]+' <<< "$line")
    valid=true
    game=$(sed "s/Game $gameId: //g" <<< "$line")

    declare -A cubes=([red]=0 [blue]=0 [green]=0)

    IFS=';' read -ra HANDS <<< "$game"
    for i in "${HANDS[@]}"; do
        IFS=',' read -ra blocks <<< "$i"
        for j in "${blocks[@]}"; do
            color=$(echo "$j" | grep -oP '[a-z]+')
            number=$(echo "$j" | grep -oP '[0-9]+')
            (( ${thresholds[$color]} < number )) && valid=false
            (( ${cubes[$color]} < number )) && cubes[$color]=$number
        done
    done
    [[ $valid == true ]] && gameIdTotals=$((gameIdTotals + ${gameId[0]}))
    powers=$((powers + $((cubes[red] * cubes[blue] * cubes[green]))))
done < input
echo "Part 1: $gameIdTotals"
echo "Part 2: $powers"
