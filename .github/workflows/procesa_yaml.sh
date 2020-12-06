#!/bin/bash

echo "Procesando fichero cc.yaml"
cc=$(ysh -f cc.yaml)
ysh -T ${cc} -q "lenguaje"
lenguaje=$(ysh -T ${cc} -q "lenguaje")
echo $lenguaje
fichero_tareas=$(ysh -T ${cc} -q "fichero_tareas")
echo $fichero_tareas
echo "CC_LENGUAJE=$lenguaje" >> CC_CONFIG
echo "CC_FICHERO_TAREAS=$fichero_tareas" >> CC_CONFIG
