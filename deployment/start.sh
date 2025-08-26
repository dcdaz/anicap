#!/bin/sh

# Author : Daniel Cordova
# Github : @dcdaz

# Small script that start, stop or restart Anicap backend

ACTION="$1"
SCRIPT_PATH=${0%/*}
CURRENT_DATE=$(date +'%Y%m%d_%k%M%S')

function start {
    $SCRIPT_PATH/backend/anicap > $SCRIPT_PATH/anicap_backend_$CURRENT_DATE.log & echo $! > $SCRIPT_PATH/.pid_file
}

function stop {
    cat .pid_file | xargs kill -9
}

if [ -z $ACTION ]; then
    echo "This script must have an action start/stop/restart"
    exit 1
fi

if [ $ACTION == "start" ]; then
    start
elif [ $ACTION == "stop" ]; then
    stop
elif [ $ACTION == "restart" ]; then
    stop
    start
fi

exit 0