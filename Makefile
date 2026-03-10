.PHONY: dev new-post

dev:
	zola serve

new-post:
	@./scripts/new_post.sh "$(slug)"
