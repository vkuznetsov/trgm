# Trigram corrector

Trigram corrector 0.1.0


## Usage
```
USAGE:
    trgm [FLAGS] --dict

FLAGS:
    -d, --dict       Name of the dictionary file
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## Example
```shell
$ cat mydict.txt
Mikhail Glinka
Pyotr Tchaikovsky
Nikolai Rimsky-Korsakov
Modest Mussorgsky
Alexander Scriabin
Sergei Rachmaninoff
Igor Stravinsky
Sergei Prokofiev
Dmitry Shostakovich
```

```shell
$ trgm -d mydict.txt
> Mihail Glinka
< Mikhail Glinka
> Sergei Rahmaninov
< Sergei Rachmaninoff
```

```shell
$ cat queries.txt
Mihail Glinka
Sergei Rahmaninov
```

```shell
$ trgm -d mydict.txt < queries.txt
Mikhail Glinka
Sergei Rachmaninoff
```
