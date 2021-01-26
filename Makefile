.PHONY: clean
.DEFAULT_GOAL:=help

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)


test: ## test
	cargo test

start: ## 启动 air 热更新
	cargo run

clean: ## 执行清理
	cargo clean

build: clean ## 打包 先格式化
	cargo build
