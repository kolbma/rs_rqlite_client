#!/bin/sh

PATH="/usr/local/bin:/usr/bin:/bin"

version="$1"
[ -n "$version" ] || version="v7.21.4"

arch="$2"
[ -n "$arch" ] || arch="$(uname -i)"

github_arch="$arch"

[ "$github_arch" = "x86_64" ] && github_arch="amd64"

sys="$(uname -s | tr 'A-Z' 'a-z')"

filename="rqlite-$version-$sys-$github_arch.tar.gz"

script_dir="$(dirname "$0")"
script_dir="$(cd "$script_dir" && pwd)"

install_dir="$script_dir/$arch"

[ "${install_dir#$script_dir}" = "$install_dir" ] && {
    echo "$0: invalid install_dir $install_dir" >&2
    exit 1
}

[ -d "$install_dir" ] || mkdir -p "$install_dir"

[ -f "$install_dir/$filename" ] || {
    url="https://github.com/rqlite/rqlite/releases/download/$version/$filename"

    wget -O "$install_dir/$filename" "$url" || {
        echo "$0: wget failed" >&2
        exit 1
    }
}

[ -d "$install_dir/${filename%.tar.gz}" ] && rm -r "$install_dir/${filename%.tar.gz}"

tar xzpf "$install_dir/$filename" -C "$install_dir/" || {
    echo "$0: untar $filename failed" >&2
    exit 1
}

[ -L "$install_dir/rqlite" ] && rm "$install_dir/rqlite"
ln -s "${filename%.tar.gz}" "$install_dir/rqlite"

[ -x "$install_dir/rqlite/rqlited" ] || {
    echo "$0: installation failed" >&2
    exit 1
}

exit 0
