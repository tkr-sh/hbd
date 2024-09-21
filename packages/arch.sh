#!/usr/bin/env sh


version_from_toml="$(rg "^version = " < Cargo.toml | sed -E 's/version = "([^"]+)"/\1/g')"
current_pkg_version="$(rg "^pkgver=" < packages/PKGBUILD | sed -E 's/pkgver=(.+)/\1/g')"

if [ "$version_from_toml" = "$current_pkg_version" ]; then
    pkgrel=$(rg "^pkgrel=" < packages/PKGBUILD | sed -E 's/pkgrel=(.+)/\1/g')
    pkgrel=$(( pkgrel + 1 ))
    sed -i "s/pkgrel=.*/pkgrel=$pkgrel/g" packages/PKGBUILD
else
    sed -i "s/pkgver=.*/pkgver=$current_pkg_version/g" packages/PKGBUILD
fi

cd packages/arch
cp ../PKGBUILD .
makepkg --printsrcinfo > .SRCINFO
git add .SRCINFO PKGBUILD
git commit -m "bump"
git push
