#!/bin/bash

echo Preparing to start server
if [ -f server.log ]; then
    echo Rotating logs
    mv server.log server.log.bak
fi
echo Server starting
./target/release/e_vent_server | tee server.log
echo Server died
