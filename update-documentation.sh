set -e

git branch gh-pages --force
git checkout gh-pages
rm -rf target/doc
cargo doc
git add target/doc --force
git commit -am "Update documentation (update-documentation.sh)"
git checkout master
