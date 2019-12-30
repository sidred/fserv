
# FSERV

fserv is file server with spa and proxy support and built with actix.

# LICENSE

MIT / APACHE

# Usage
```
fserv -p <port> -i <index_file> -d <directory> -f <path>=<target> -f <path>=<target>
```

For help
```
fserv --help

```

# Example

With the following folder structure
```
.
├── html_data/
│   └── index.html
└── build/
    ├── main.js
    └── style.css

```

Use the command
```
fserv -p 9000 -i html_data/index.html  -d build -f api=http://localhost:9010 -f ws=http://localhost:9020
```

This will start the server on port 9000 and server the files in build directory. Requests to /api and /ws will be proxied 

| Request                      | Response                                                  |
| -----------------------------|-----------------------------------------------------------|
| localhost:9000/              | html_data/index.html  file                                |
| localhost:9000/main.js       | build/main.js file                                        |
| localhost:9000/style.css     | build/style.css file                                      | 
| localhost:9000/unknown.png   | html_data/index.html file since the file does not exist   | 
| localhost:9000/user/info     | html_file/index.html file                                 |
| localhost:9000/api/list      | proxy the request to http://localhost:9010/api/list       |
| localhost:9000/ws/updates    | proxy the request to http://localhost:9020/ws/updates     |

# Config

```
USAGE:
    fserv [FLAGS] [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -n, --no-spa
            Disables spa mode. When spa mode is enabled an index.html (configured by -i flag) file is returned when a
            resourece is not found When spa mode is disabled a 404 error is returned when a resource is not found
    -V, --version
            Prints version information

OPTIONS:
    -d, --directory <directory>
            Directory to serve. Uses current directory by default [default: .]

    -i, --index-file <index-file>
            The file to server if when a resource is not found and spa mode is enabled [default: index.html]

    -p, --port <port>
            Sets the port [default: 8000]

    -f, --forward <proxies>...
            Optional path to forward as a key=value pair. Multiple entries can be added

    -w, --workers <workers>
            Sets the number of worker threads [default: 1]

```
