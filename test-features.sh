#!/bin/sh

features="migration migration_embed ureq ureq_tls ureq_webpki"
features_addon="log percent_encoding tracing ureq_charset ureq_socks_proxy url"

cargo_test() {
    feature="$1"
    addon="$2"

    rc=0
    if [ -n "$feature" ] ; then
        echo "$0: cargo test --no-default-features -F \"$feature $addon\""
        cargo test --no-default-features -F "$feature $addon"
        rc="$?"
    else
        echo "$0: cargo test"
        cargo test
        rc="$?"
    fi

    [ "$rc" -eq 0 ] || {
        echo "0: failed feature $f" >&2
        exit "$rc"
    }
}

script_dir="$(dirname "$0")"
script_dir="$(cd "$script_dir" && pwd)"

default_features=$(grep "^default =" "$script_dir/Cargo.toml" | cut -d '[' -f 2 | tr -d '"[]')

cargo_test

for f in $features ; do
    cargo_test "$f"
    for fa in $features_addon ; do
        [ "X$default_features" = "X$f, $fa" ] && continue
        cargo_test "$f" "$fa"
    done
done

exit 0
