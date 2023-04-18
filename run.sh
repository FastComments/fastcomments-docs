#!/bin/bash

env $(cat /home/winrid/fastcomments-docs/env) node /home/winrid/fastcomments-docs/src/server-search-engine.js >> /tmp/fastcomments-docs-search.log 2>&1;
