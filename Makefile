# Yes, I'm using make for Rust instead of cargo. Shut up.

BIN_NAME := 12-days
DEV_BIN_NAME := devbuild

$(BIN_NAME): main.rs
	rustc -O main.rs -o $@

$(DEV_BIN_NAME): main.rs
	rustc -g main.rs -o $@

clean:
	if [ -f $(BIN_NAME) ]; then rm $(BIN_NAME); fi
	if [ -f $(DEV_BIN_NAME) ]; then rm $(DEV_BIN_NAME); fi

