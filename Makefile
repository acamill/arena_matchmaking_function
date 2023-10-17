# include .env file and export its env vars
# (-include to ignore error if it does not exist)
-include .env

.PHONY: build clean publish test

# Variables
CARGO_NAME=arena-matchmaking-function
DOCKER_IMAGE_NAME ?= acammm/${CARGO_NAME}

check_docker_env:
ifeq ($(strip $(DOCKER_IMAGE_NAME)),)
	$(error DOCKER_IMAGE_NAME is not set)
else
	@echo DOCKER_IMAGE_NAME: ${DOCKER_IMAGE_NAME}
endif

# Default make task
all: build

docker_build: 
	docker buildx build --platform linux/amd64 -f Dockerfile -t ${DOCKER_IMAGE_NAME}:v1 --load ./
docker_publish: 
	docker buildx build --platform linux/amd64 -f Dockerfile -t ${DOCKER_IMAGE_NAME}:v1 --push ./

build: docker_build measurement


publish: docker_publish measurement

measurement: check_docker_env
	@docker run -d --platform=linux/amd64 -q --name=my-switchboard-function ${DOCKER_IMAGE_NAME}:v1
	@docker cp my-switchboard-function:/measurement.txt ./measurement.txt
	@echo -n 'MrEnclve: '
	@cat measurement.txt
	@docker stop my-switchboard-function > /dev/null
	@docker rm my-switchboard-function > /dev/null

# Task to clean up the compiled rust application
clean:
	cargo clean
