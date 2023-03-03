publish:
	rm -rf dist/
	cp .pypirc ~/
	maturin build --release --manylinux off --out dist
	maturin upload --skip-existing -r bink dist/*
