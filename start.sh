#!/bin/bash

sudo cp fastcomments-docs.service /etc/systemd/system/
sudo systemctl enable fastcomments-docs
sudo systemctl restart fastcomments-docs
