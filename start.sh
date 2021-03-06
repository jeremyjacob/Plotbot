#!/bin/bash

cd /home/slicer/Plotbot/GCode
echo "Killing previous sessions"
tmux kill-session -t GCode
tmux kill-session -t Caddy
tmux new-session -d -s "GCode" ./plotbot-server
cd ../Caddy
tmux new-session -d -s "Caddy" ./caddy.sh
echo "Started:"
sleep 0.3
echo $(tmux ls)
