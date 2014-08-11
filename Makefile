.SILENT:

clean:
	## Remove documentation, library and tests
	cargo clean

doc:
	## Create documentation
	cargo doc

help:
	## Show this help
	grep -A1 ^[a-z].*\: Makefile | sed -r 's/: (.*)$$/:/g' | sed ':a;N;$$!ba;s/:\n//g' | sed s,\\#,\\t,g | grep -v \\--

test:
	## Run tests
	cargo test
	cargo build && LD_LIBRARY_PATH=$(PWD)/target rustdoc --test -L target README.md

trailing: $(SRC)
	# Check trailing spaces
	@NB=0; for FI in $(SRC); do \
		grep -n '\s\+$$' $$FI; RET=$$?; \
		if test "$$RET" == "0"; then \
			echo $$FI; \
			NB=1; \
		fi; \
	done; exit $$NB

version:
	## Display version of source code
	git describe

