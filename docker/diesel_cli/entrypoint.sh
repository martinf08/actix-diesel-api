#!/usr/bin/env bash
diesel setup
diesel migration run
tail -f /dev/null