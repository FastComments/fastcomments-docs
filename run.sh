#!/bin/bash

# Load NVM
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

env $(cat /home/winrid/fastcomments-docs/env) node /home/winrid/fastcomments-docs/src/server-search-engine.js >> /tmp/fastcomments-docs-search.log 2>&1;
