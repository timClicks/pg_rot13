# pg_rot13

Applies the [ROT13] (rotate 13) pseudo-encryption algorithm to text input.
It is a minimal Postgres extension written in the [`pgx`] extension framework.

`pg_rot13` has been tested against Postgres versions 10-13 on the Linux/AMD64 platform.

[`pgx`]: https://github.com/zombodb/pgx
[ROT13]: https://en.wikipedia.org/wiki/ROT13

## Usage

The first step is to load the extension within a running Postgres instance:

```sql
> CREATE EXTENSION pg_rot13;
CREATE EXTENSION
```

From here, you can use the `rot13()` function within a SELECT statement:

```sql
> SELECT rot13('hello');
 rot13 
-------
 uryyb
 (1 row)
```

**Note:** if you're running commands interactively, remember that SQL requires
that string literals are quoted with single-quotes ('). Double-quotes (")
indicate table names.

ROT13 only applies to characters within the `[A-Za-z]` range. Other
characters will be left as-is.

```sql
> SELECT rot13('Grüß Gott');
 rot13 
-------
 Teüß Tbgg
 (1 row)
```

## Installation

At the moment, you need to compile the extension and install it yourself. This depends on having [`pgx`] installed and a working Postgres installation that can be detected by `pg_config`.


1. Clone the source code repository:
    ```console
    git clone https://github.com/timClicks/pg_rot13.git
    ```
1. Move into the newly-created directory:
   ```console
   cd pg_rot13
   ```
1. Ask `pgx` to compile and install the extension:
   ```console
   cargo pgx install
   ````

## License

Dual licensed under the [Blue Oak Model License 1.0.0] and the [Apache License 2.0]. 

[Blue Oak Model License 1.0.0]: https://blueoakcouncil.org/license/1.0.0
[Apache License 2.0]: https://www.apache.org/licenses/LICENSE-2.0.html