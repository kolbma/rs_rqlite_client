#!/bin/sh

PATH="/usr/local/bin:/usr/bin:/bin"

script_dir="$(dirname "$0")"
script_dir="$(cd "$script_dir" && pwd)"

arch="$(uname -i)"
github_arch="$arch"
[ "$github_arch" = "x86_64" ] && github_arch="amd64"

install_dir="$script_dir/$arch"

[ "${install_dir#$script_dir}" = "$install_dir" ] && {
    echo "$0: invalid install_dir $install_dir" >&2
    exit 1
}

[ -x "$install_dir/rqlite/rqlited" ] || {
    echo "$0: rqlited $install_dir/rqlite/rqlited not found" >&2
    exit 1
}


db_data_dir_base="$script_dir/../target/rqlite_data_"
db_data_dir="${db_data_dir_base}${RANDOM}${RANDOM}"

rqlited_pid() {
    pid=""
    lsof -n -t -w -- "$db_data_dir_base"*/raft.db >/dev/null 2>&1
    if [ "$?" -eq 0 ] ; then
        pid=$(lsof -n -t -w -- "$db_data_dir_base"*/raft.db 2>/dev/null)
    else
        pid=$(ps -e | sed -e '/rqlited/!d' -e 's/^[ ]*\([0-9]*\)[^0-9]*.*$/\1/')
    fi
    
    [ -n "$pid" ] && echo "$pid" || {
        #echo "$0: pid failed" >&2
        return 1
    }

    return 0
}

rqlited_status() {
    pid=$(rqlited_pid)

    [ "$?" -eq 0 ] && [ -n "$pid" ] || return 1

    readlink -e "/proc/$pid/exe" | grep -q "/rqlited$"
    return "$?"
}



if [ "X$1" = "X--stop" ] ; then
    echo "$0: stopping rqlited ..."
    pid=$(rqlited_pid)
    [ -n "$pid" ] || {
        echo "$0: rqlited not running"
        exit 0
    }
    kill "$pid"
    sleep 1
    rqlited_status
    [ "$?" -ne 0 ] || {
        echo "$0: stop failed" >&2
        exit 1
    }
    exit 0
fi


[ -d "$db_data_dir" ] || {
    mkdir -p "$db_data_dir" || {
        echo "$0: creating $db_data_dir failed" >&2
        exit 1
    }
}

db_data_dir=$(cd "$db_data_dir" && pwd)
echo "$db_data_dir" | grep -q "/rqlite_data_"
[ "$?" -eq 0 ] || {
    echo "$0: db_data_dir failed" >&2
    exit 1
}


echo "$0: starting rqlited ..."
{ "$install_dir/rqlite/rqlited" -bootstrap-expect 1 "$db_data_dir" ; rm -r "$db_data_dir" ; } &
sleep 2

rqlited_status
if [ "$?" -eq 0 ] ; then
    echo "$0: rqlited started in background"
else
    echo "$0: rqlited failed" >&2
    exit 1
fi

exit 0
