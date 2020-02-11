mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
base_dir := $(notdir $(patsubst %/,%,$(dir $(mkfile_path))))

SERVICE ?= $(base_dir)
GIT_HASH := $(shell git rev-parse HEAD)

DOCKER_REGISTRY=docker.io
DOCKER_REPOSITORY_NAMESPACE=kentakudo
DOCKER_REPOSITORY_IMAGE=$(SERVICE)
DOCKER_REPOSITORY=$(DOCKER_REGISTRY)/$(DOCKER_REPOSITORY_NAMESPACE)/$(DOCKER_REPOSITORY_IMAGE)
DOCKER_IMAGE_TAG=$(GIT_HASH)

.PHONY: docker-image
docker-image:
	docker build -t $(DOCKER_REPOSITORY):$(DOCKER_IMAGE_TAG) . \
		--build-arg SERVICE=$(SERVICE)

.PHONY: docker-auth
docker-auth:
	@docker login -u $(DOCKER_ID) -p $(DOCKER_PASSWORD) $(DOCKER_REGISTRY)

.PHONY: docker-build
docker-build: docker-image docker-auth
	docker tag $(DOCKER_REPOSITORY):$(DOCKER_IMAGE_TAG) $(DOCKER_REPOSITORY):latest
	docker push $(DOCKER_REPOSITORY)
