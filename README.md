# scurl -XGET "server URL"

## server directives

```
$ scurl -s start -f config.json
$ scurl -s stop
$ scurl -s reload -f config.json
$ scurl -s show
$ scurl -s save
$ scurl -s watch
```

## usage examples

```
# mock a GET api that responses with a `OK`
$ scurl -XGET localhost:8080/api/health -d'OK'

# mock a POST api that responses the original post payload
$ scurl -XPOST -d'$.' localhost:8080/api/items

# mock a GET api that responses a json object
$ scurl -XGET -d'{"id": 2}' localhost:8080/api/items/2

# mock a GET api that responses a json object respect the path variables
$ scurl -XGET -d'{"id=1": {"id": 1}}' localhost:8080/api/items/:id

# mock a GET api that returns NOT_FOUND status codes
$ scurl -XGET -d'{"id": 2}' localhost:8080/api/items/2
```

## arguments

```
scurl -h
```