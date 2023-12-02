#!/bin/bash
#
gameIdTotals=0
powers=0
declare -A thresholds
thresholds=([red]=12 [blue]=14 [green]=13)
while read -r line; do
    gameId=( $(echo "$line" | grep -oP '[0-9]+') )
    valid=true
    game=$(echo "$line" | sed "s/Game ${gameId[0]}: //g")

    declare -A cubes
    cubes=([red]=0 [blue]=0 [green]=0)

    IFS=';' read -ra HANDS <<< "$game"
    for i in "${HANDS[@]}"; do
        IFS=',' read -ra blocks <<< "$i"
        for j in "${blocks[@]}"; do
            color=$(echo "$j" | grep -oP '[a-z]+')
            number=$(echo "$j" | grep -oP '[0-9]+')
            if [[ ${thresholds[$color]} -lt $number ]]; then
                valid=false
            fi
            if [[ ${cubes[$color]} -lt $number ]]; then
                cubes[$color]=$number
            fi
        done
    done
    if [[ $valid == true ]]; then
        gameIdTotals=$((gameIdTotals + ${gameId[0]}))
    fi
    power=$((cubes[red] * cubes[blue] * cubes[green]))
    powers=$((powers + power))
done < input
echo "$gameIdTotals"
echo "$powers"
