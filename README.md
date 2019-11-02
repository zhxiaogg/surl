# surl "server URL"

## server directives

```
$ surl -s start -f config.json
$ surl -s stop
$ surl -s reload -f config.json
$ surl -s show
$ surl -s save
$ surl -s watch
```

## request examples

```
# mock a GET api that responses with a `OK`
$ surl -XGET localhost:8080/api/health -d'OK'

# mock a POST api that responses the original post payload
$ surl -XPOST -d'$.' localhost:8080/api/items

# mock a GET api that responses a json object
$ surl -XGET -d'{"id": 2}' localhost:8080/api/items/2

# mock a GET api that responses a json object respect the path variables
$ surl -XGET -d'{"id=1": {"id": 1}}' localhost:8080/api/items/:id

# mock a GET api that returns NOT_FOUND status codes
$ surl -XGET -d'{"id": 2}' localhost:8080/api/items/2
```

## arguments

```
surl -h
```