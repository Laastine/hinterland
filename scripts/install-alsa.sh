#!/bin/bash

sudo apt-get remove --purge alsa-base pulseaudio
sudo apt-get install alsa-base pulseaudio
sudo alsa force-reload
sudo apt install libasound2-dev
