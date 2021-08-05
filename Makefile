publish:
	maturin build --no-sdist --release --manylinux off --out dist
	maturin upload --skip-existing -u rusty-pypi -p Sus4yNWcs9hnmqW-DuW1 -r https://git.bink.com/api/v4/projects/414/packages/pypi dist/*
