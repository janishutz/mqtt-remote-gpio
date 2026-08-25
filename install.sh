#!/bin/sh

cargo install
sudo cp ./config.yml /etc/mqtt-remote-gpio.config.yml
sudo cp ./mqtt-remote-gpio.service /etc/systemd/system/
