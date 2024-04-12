@echo off
if "%1"=="" (
    set SERVER=localhost
)

set SERVER=%1

systeminfo /S %SERVER%
