SHELL := /bin/bash
HIDE ?= @

-include Makefile.util.mk

.PHONY: build

name := "app_blocks_api"
port := 15039

## ALIYUN_REGISTRY, ALIYUN_USERNAME, ALIYUN_REGISTRY_PWD,

clean:
	$(HIDE)cargo clean

fix:
	$(HIDE)cargo fmt

gen:
	$(HIDE)which sea-orm-cli >/dev/null || cargo install sea-orm-cli
	$(HIDE)sea-orm-cli generate entity -o src/infra/db/entity
	$(HIDE)sea-orm-cli generate entity -o migration/src/entity

build:
	$(HIDE)cargo build

test:
	$(HIDE)cargo test

docker-build:
	$(HIDE)docker build -f Dockerfile -t $(name) .

docker-push: env := "dev"
docker-push: version := $(shell date +%y%m%d%H%M%S)
docker-push: tag := $(name):$(env)-$(version)
docker-push:
	$(HIDE)echo $(tag)
	$(HIDE)docker build -f Dockerfile -t $(tag) .
	$(HIDE)echo "$(ALIYUN_REGISTRY_PWD)" | docker login $(ALIYUN_REGISTRY) --username $(ALIYUN_USERNAME) --password-stdin
	$(HIDE)docker tag $(tag) $(ALIYUN_REGISTRY)/weworkci/$(tag)
	$(HIDE)docker push $(ALIYUN_REGISTRY)/weworkci/$(tag)
	$(HIDE)echo $(ALIYUN_REGISTRY)/weworkci/$(tag)

docker-up: image := "registry.cn-hangzhou.aliyuncs.com/penghu/app_blocks_api:dev-231230210046"
docker-up:
	$(HIDE)docker run -it --rm --name $(name) --env-file .env --publish $(port):$(port) $(image) sh

docker-down:
	$(HIDE)docker rm -f $(name)
