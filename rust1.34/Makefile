USER=docker.io/actionloop
IMAGE=actionloop-rust-v1.32

.PHONY: build

build:
	docker build -t $(USER)/$(IMAGE) .

devel: build
	docker run -ti -p 8080:8080 --entrypoint=bash \
	-v $(PWD):/mnt -e OW_COMPILER=/mnt/compile \
	$(USER)/$(IMAGE)

push: build
	docker login
	docker push $(USER)/$(IMAGE)
