#!/bin/bash

# Load NVM and use project-pinned Node.js version
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
nvm use

env $(cat /home/winrid/fastcomments-docs/env) node /home/winrid/fastcomments-docs/src/server-search-engine.js >> /tmp/fastcomments-docs-search.log 2>&1;
