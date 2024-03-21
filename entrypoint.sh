#!/bin/bash
cron
exec "$@"
tail -f /dev/null  # Keep the script running
