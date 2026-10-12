.PHONY: dev new-post

dev:
	zola serve

build:
	git submodule update --init --recursive
	docker build -t thought-repo .

run-docker:
	docker run --rm -p 8080:80 thought-repo

new-post:
	@./scripts/new_post.sh "$(slug)"
