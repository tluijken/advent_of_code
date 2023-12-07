#!/bin/bash
mapfile -t lines < "$1"
timeline=($([[ $2 == "true" ]] && sed "s/Time: //g; s/[[:space:]]//g" <<< "${lines[0]}" || echo "${lines[0]#Time: }"))
distances=($([[ $2 == "true" ]] && sed "s/Distance: //g; s/[[:space:]]//g" <<< "${lines[1]}" || echo "${lines[1]#Distance: }"))
m=1;
for (( i=0; i<${#timeline[@]}; i++ )); do
    o=0
    for (( t = 0; t < ${timeline[i]}; t++ )); do (( t * (timeline[i] - t) > distances[i] )) && (( o++ )); done
    (( o > 0 )) && (( m *= o ))
done
echo $m
