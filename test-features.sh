#!/bin/sh

features="migration migration_embed ureq ureq_tls ureq_webpki"
features_addon="log monitor percent_encoding tracing ureq_charset ureq_socks_proxy url"


cargo_test() {
    feature="$1"
    addon="$2"
    all_features="$3"

    rc=0
    if [ -n "$feature" ] ; then
        [ -n "$clippy" ] && {
            echo "$0: $clippy --no-deps --no-default-features -F \"$feature $addon\" -- -Dwarnings"
            $clippy --no-deps --no-default-features -F "$feature $addon" -- -Dwarnings
            rc="$?"
        }

        [ "$rc" -eq 0 ] && {
            echo "$0: cargo test --no-default-features -F \"$feature $addon\""
            cargo test --no-default-features -F "$feature $addon"
            rc="$?"
        }

        [ "$rc" -eq 0 ] && {
            echo "$0: cargo test --no-default-features -F \"$feature $addon\" --doc -- --nocapture"
            RUSTFLAGS="-D warnings" cargo test --no-default-features -F "$feature $addon" --doc -- --nocapture
            rc="$?"
        }
    else
        [ -n "$all_features" ] && all_features="--all-features"

        [ -n "$clippy" ] && {
            echo "$0: $clippy --no-deps $all_features -- -Dwarnings"
            $clippy --no-deps $all_features -- -Dwarnings
            rc="$?"
        }

        [ "$rc" -eq 0 ] && {
            echo "$0: cargo test $all_features"
            cargo test $all_features
            rc="$?"
        }

        [ "$rc" -eq 0 ] && {
            echo "$0: cargo test $all_features --doc -- --nocapture"
            RUSTFLAGS="-D warnings" cargo test $all_features --doc -- --nocapture
            rc="$?"
        }
    fi

    [ "$rc" -eq 0 ] || {
        f="$all_features"
        [ -n "$feature" ] && f="$feature $addon"
        echo "0: failed feature $f" >&2
        exit "$rc"
    }
}


script_dir="$(dirname "$0")"
script_dir="$(cd "$script_dir" && pwd)"

default_features=$(grep "^default =" "$script_dir/Cargo.toml" | cut -d '[' -f 2 | tr -d '"[]')

clippy="cargo clippy"
$clippy -h >/dev/null 2>&1
[ "$?" -eq 0 ] || clippy=""

cargo_test

cargo_test "" "" --all-features

for f in $features ; do
    cargo_test "$f"
    for fa in $features_addon ; do
        [ "X$default_features" = "X$f, $fa" ] && continue
        cargo_test "$f" "$fa"
    done
done

exit 0
