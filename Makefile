.PHONY: clean run sync
all:
	@cargo build --release

clean:
	@rm -r target

run:
	@cd src && cargo run --release

sync:
	@rsync -avzP . --exclude "target/" --exclude ".git/" \
				   --exclude ".idea/" bigdata:~/parknn
