publish:
	rm -rf dist/
	maturin build --no-sdist --release --manylinux off --out dist
	for file in $$(ls dist/*.whl); do mv -v $$file $$(echo $$file | perl -p -e 's/10_\d+/11_0/g'); done
	maturin upload --skip-existing -u c02bed7a-4224-4fb4-82ad-17499e4d0e4a -p 8134bbb1-2bb8-4895-9c57-9ba4f2196ad1 -r https://pypi.uksouth.bink.sh/simple dist/*
