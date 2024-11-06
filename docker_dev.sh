#!/bin/bash

APP_NAME="rust_rest_api"
IMAGE_NAME="${APP_NAME}_image"
CONTAINER_NAME="${APP_NAME}_container"

# Build the Docker image
build() {
  echo "Building Docker image..."
  docker build -t $IMAGE_NAME .
}

# Run the Docker container with a bind mount
run() {
  echo "Running Docker container with bind mount..."
  docker run -d --name $CONTAINER_NAME \
    -v "$(pwd)":/usr/src/app \
    -p 8000:8080 \
    $IMAGE_NAME
}

# Stop and remove the Docker container
stop() {
  echo "Stopping and removing Docker container..."
  docker stop $CONTAINER_NAME
  docker rm $CONTAINER_NAME
}

# Cleanup unused images and containers
cleanup() {
  echo "Cleaning up unused Docker images and containers..."
  docker system prune -f
}

# Help menu
help() {
  echo "Usage: $0 {build|run|stop|cleanup|help}"
}

# Main script to handle different commands
case "$1" in
  build)
    build
    ;;
  run)
    run
    ;;
  stop)
    stop
    ;;
  cleanup)
    cleanup
    ;;
  help|*)
    help
    ;;
esac
