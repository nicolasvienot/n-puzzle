#!bin/sh

# This script will clean docker
# Careful this will clean everything that is not running or attached to a running image

docker system prune
docker image prune -a
docker volume prune