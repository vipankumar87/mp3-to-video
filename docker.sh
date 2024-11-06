#!/bin/bash

APP_NAME="rust_rest_api"
IMAGE_NAME="${APP_NAME}_image"
CONTAINER_NAME="${APP_NAME}_container"

# Build the Docker image
build() {
  echo "Building Docker image..."
  docker build -t $IMAGE_NAME .
}

# Run the Docker container
run() {
  echo "Running Docker container..."
  docker run -d --name $CONTAINER_NAME -p 8000:8000 -e DATABASE_URL="sqlite:///app/rust_rest_api.db" $IMAGE_NAME
}

# Stop and remove the Docker container
stop() {
  echo "Stopping and removing Docker container..."
  docker stop $CONTAINER_NAME
  docker rm $CONTAINER_NAME
}

# Show logs for the running container
logs() {
  echo "Displaying logs..."
  docker logs -f $CONTAINER_NAME
}

# Access the running container shell
shell() {
  echo "Accessing container shell..."
  docker exec -it $CONTAINER_NAME /bin/bash
}

# Run diesel migrations inside the container
migrate() {
  echo "Running migrations..."
  docker exec -it $CONTAINER_NAME diesel migration run
}

# Cleanup unused images and containers
cleanup() {
  echo "Cleaning up unused Docker images and containers..."
  docker system prune -f
}

# Help menu
help() {
  echo "Usage: $0 {build|run|stop|logs|shell|migrate|cleanup|help}"
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
  logs)
    logs
    ;;
  shell)
    shell
    ;;
  migrate)
    migrate
    ;;
  cleanup)
    cleanup
    ;;
  help|*)
    help
    ;;
esac
