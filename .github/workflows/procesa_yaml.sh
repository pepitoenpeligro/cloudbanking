#!/bin/bash

echo "Procesando fichero cc.yaml"
lenguaje=$(ysh -f cc.yaml -q "lenguaje")
[[ $lenguaje =~ \= ]] && echo "«lenguaje» no es una cadena" && exit 1
fichero_tareas=$(ysh -f cc.yaml -q "fichero_tareas")
[[ $fichero_tareas =~ \= ]] && echo "«fichero_tareas» no es una cadena" && exit 2
echo "CC_LENGUAJE=$lenguaje" >> CC_CONFIG
echo "CC_FICHERO_TAREAS=$fichero_tareas" >> CC_CONFIG
