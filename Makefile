build-3.9:
	maturin build --no-sdist --release --manylinux off -i $(shell pyenv root)/versions/3.9.*/bin/python --out dist
build-3.8:
	maturin build --no-sdist --release --manylinux off -i $(shell pyenv root)/versions/3.8.*/bin/python --out dist
build-3.7:
	maturin build --no-sdist --release --manylinux off -i $(shell pyenv root)/versions/3.7.*/bin/python --out dist
publish: build-3.9 build-3.8 build-3.7
	maturin upload --skip-existing -u rusty-pypi -p Sus4yNWcs9hnmqW-DuW1 -r https://git.bink.com/api/v4/projects/414/packages/pypi dist/*
