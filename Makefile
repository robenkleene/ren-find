.PHONY: show-man

show-man:
	man $$(ls -t target/debug/build/ren-find-*/out/ren.1 | head -1)
