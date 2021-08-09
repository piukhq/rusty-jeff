publish:
	rm -rf dist/
	maturin build --no-sdist --release --manylinux off --out dist
	for file in $$(ls dist/*.whl); do mv -v $$file $$(echo $$file | perl -p -e 's/10_\d+/11_0/g'); done
	maturin upload --skip-existing -u rusty-pypi -p Sus4yNWcs9hnmqW-DuW1 -r https://git.bink.com/api/v4/projects/414/packages/pypi dist/*
