[![crates.io](https://img.shields.io/crates/v/rqlite_client)](https://crates.io/crates/rqlite_client)
[![docs](https://docs.rs/rqlite_client/badge.svg)](https://docs.rs/rqlite_client)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)
[![Rust](https://github.com/kolbma/rs_rqlite_client/actions/workflows/rust.yml/badge.svg)](https://github.com/kolbma/rs_rqlite_client/actions/workflows/rust.yml)

# rqlite_client v0.0.1-alpha.14


This is an __rqlite__ database client library with optional extra _convenience_.

__rqlite__ is an easy-to-use, lightweight, distributed relational database, which uses SQLite as its storage engine.
It is super-simple to deploy, operating it is very straightforward, and its clustering capabilities provide you
with fault-tolerance and high-availability.

See the documentation of __rqlite__ database for a [Quick start](https://rqlite.io/docs/quick-start/)!

__`rqlite_client`__ provides a type safe __Rust library API__ to use some __rqlite__ database backend from your code.
There is the possibility to create type safe queries and retrieve type safe results.
It has an optional implementation for database scheme migration and rollback.
Per default the HTTP(S) requests are handled by a provided
[`RequestBuilder`](https://docs.rs/rqlite_client/latest/rqlite_client/trait.RequestBuilder.html)
implementation based on crate [`ureq`](https://crates.io/crates/ureq).
But you can provide any implementation yourself for supporting your preferred HTTP client.
The crate supports [`log`](https://crates.io/crates/log) or [`tracing`](https://crates.io/crates/tracing).

See [Usage](#usage) and [Examples](#examples) for further information!


## Features

* __default = \["monitor", "ureq", "url"\]__

* `log`

    Uses [`log`](https://crates.io/crates/log) for some logging. Logger need to be configured via `log` crate
    in your application code.
    <br><br>

* `migration`

    Enables support for schema migration of __rqlite__ database.
    See [`Migration`](https://docs.rs/rqlite_client/latest/rqlite_client/migration/struct.Migration.html).
    <br><br>

* `migration_embed`

    Enables schema migration support with embedding SQL from files in the application code.
    See [`Migration`](https://docs.rs/rqlite_client/latest/rqlite_client/migration/struct.Migration.html).
    <br><br>

* `monitor`

    Enables monitor endpoints.
    See [Monitor](https://docs.rs/rqlite_client/latest/rqlite_client/monitor/index.html).
    <br><br>

* `percent_encoding`

    If you disable feature `url`, you have to add feature `percent_encoding` to get working _GET_ _SELECT_ queries.
    <br><br>

* `tracing`

    Uses [`tracing`](https://crates.io/crates/tracing) for some logging. Tracing need to be configured
    via `tracing` crate in your application code.
    <br><br>

* `ureq`

    The default HTTP client used for communication with the __rqlite__ database. If you disable the feature,
    you have to provide an
    own [`RequestBuilder`](https://docs.rs/rqlite_client/latest/rqlite_client/trait.RequestBuilder.html)
    implementation to handle your replacement
    of [`Request`](https://docs.rs/rqlite_client/latest/rqlite_client/struct.Request.html).
    <br><br>

* `ureq_charset`

    Enables Non-UTF8 charset handling in `ureq` requests.
    <br><br>

* `ureq_socks_proxy`

    Enables support of __Socks Proxy__ Urls in `ureq`.
    <br><br>

* `ureq_tls`

    Enables __TLS__ support for `ureq`-requests with loading certs from system store.
    <br><br>

* `ureq_webpki`

    Enables __TLS__ support for `ureq`-requests with only embedded Mozilla cert store.
    <br><br>

* `url`

    Uses per default [`url::Url`](https://docs.rs/url/latest/url/struct.Url.html) instead of string
    manipulation and [`percent_encoding`](https://docs.rs/percent_encoding).


## Support, Issues, Contributing

__`rqlite_client`__ is an __Open Source__ project and everybody should be encouraged to contribute
with the individual possibilities to improve the project and its results.

If you need __help__ with your development based on this library, you may ask questions and for assistance in the
__[Github discussions](https://github.com/kolbma/rs_rqlite_client/discussions)__.

For any assistance with __rqlite__ database, you are requested to get in contact with
the __rqlite__ project at __<https://rqlite.io/community/>__.

You are free to create meaningful and reproducable __issue reports__ in
__[Github issues](https://github.com/kolbma/rs_rqlite_client/issues)__.
Please be kind, tolerant and forbear with developers providing you a beneficial product,
although it isn't everybodys flavour and can't be perfect :wink:

### Code contribution

You can provide _Pull-Requests_ to the __[Github main branch](https://github.com/kolbma/rs_rqlite_client)__.

It is preferred that there are no warnings with __`warn(clippy::pedantic)`__ and the build needs to
be successful with __`forbid(unsafe_code)`__.

The __stable-toolchain__ is used for development, but the _crate_ should compile back to
specified __MSRV__ in _Cargo.toml_.


## Running tests

For running tests you'll need a working __rqlited__ installation in subdirectory _rqlite_.
There is an `rqlite/install.sh` script which could do the installation for your environment, if the environment
is already supported in the script.

The test routines are looking up __rqlited__ in a directory structure like _rqlite/&lt;arch&gt;/rqlite/rqlited_.

For testing make use of __`cargo`__.

```bash
$ cargo test --all-features
```

There is also a shell script `./test-features.sh` to test all __meaningful__ feature __combinations__.

```bash
$ ./test-features.sh
```

Before crate release the tests need to be successful in __all combinations__ with the
cargo __addon__ __test-all-features__:
```bash
$ cargo test-all-features
```


## Usage

### Url encoding

By default the enabled __feature__ `url` handles url encoding. If you don't want to use `url` dependency, there is the possibility
to enable __feature__ `percent_encoding` for handling url encoding.
__*One or the other needs to be enabled or the generated urls won't be correct!*__

### Database scheme migration and rollback support

If you want to use database scheme migration and rollback, you have to enable `migration` or `migration_embed` __feature__.
See [`Migration`](https://docs.rs/rqlite_client/latest/rqlite_client/migration/struct.Migration.html)
for further documentation.

### Logging

`rqlite_client` does some logging if there is enabled the __feature__ `log` or `tracing` and the crates has been initialised
for logging.


## Examples

### Query database

A simple query of your local database might look like...

```rust

use std::time::Duration;

use rqlite_client::{
    request_type::Get, response, Connection, Mapping, Query, Request,
    RequestBuilder, response::Result,
};

let url = "http://localhost:4001";

##[cfg(feature = "url")]
let con = Connection::new(url).expect("url failed");
##[cfg(not(feature = "url"))]
let con = Connection::new(url);

let query = con
    .query()
    .set_sql_str_slice(&["SELECT COUNT(*) FROM tbl WHERE col = ?", "test"]);

let result = response::query::Query::try_from(query.request_run().unwrap());

if let Ok(response) = result {
    if let Some(Mapping::Standard(success)) = response.results().next() {
        let row = 0;
        let col = 0;
        if let Some(rows_found) = &success.value(row, col) {
            println!("tbl has {rows_found} row(s)");
        }
    }
}
```

See [`Query`](https://docs.rs/rqlite_client/latest/rqlite_client/struct.Query.html) for further documentation.


### Insert data

To insert data you have to use
[`Connection::execute()`](https://docs.rs/rqlite_client/latest/rqlite_client/struct.Connection.html#method.execute)
and [`request_type::Post`](https://docs.rs/rqlite_client/latest/rqlite_client/request_type/struct.Post.html).

```rust

use std::time::Duration;

use rqlite_client::{
    request_type::Post, response, Connection, Mapping, Query, Request,
    RequestBuilder, response::Result,
};

let url = "http://localhost:4001";

##[cfg(feature = "url")]
let con = Connection::new(url).expect("url failed");
##[cfg(not(feature = "url"))]
let con = Connection::new(url);

let query = con
    .execute()
    .push_sql_str_slice(&["INSERT INTO tbl (col) VALUES (?)", "test"]);

let result = response::query::Query::try_from(query.request_run().unwrap());

if let Ok(response) = result {
    if let Some(Mapping::Execute(success)) = response.results().next() {
        println!("last inserted primary key {}", success.last_insert_id);
    }
}
```

See [`Query`](https://docs.rs/rqlite_client/latest/rqlite_client/struct.Query.html) for further documentation.


## Current version

[Source https://github.com/kolbma/rs_rqlite_client/tree/v0.0.1-alpha.14](https://github.com/kolbma/rs_rqlite_client/tree/v0.0.1-alpha.14)  
[Download https://github.com/kolbma/rs_rqlite_client/releases/tag/v0.0.1-alpha.14](https://github.com/kolbma/rs_rqlite_client/releases/tag/v0.0.1-alpha.14)

## License

__LGPL-2.1-only [LICENSE.md](LICENSE.md)__

__rqlite_client - rqlite database client__  
__Copyright (C) 2023  Markus Kolb__

This library is free software; you can redistribute it and/or  
modify it under the terms of the GNU Lesser General Public  
License as published by the Free Software Foundation; either  
version 2.1 of the License, or (at your option) any later version.  

This library is distributed in the hope that it will be useful,  
but WITHOUT ANY WARRANTY; without even the implied warranty of  
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU  
Lesser General Public License for more details.  

You should have received a copy of the GNU Lesser General Public  
License along with this library; if not, write to the Free Software  
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
