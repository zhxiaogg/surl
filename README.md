# surl "server URL"

The api/server mocking tool that you'll fall in love in seconds.

## server directives

```
$ surl server start
$ surl server stop
$ surl server list (TODO)
$ surl server watch (TODO)
```

## request examples

```
# mock a GET api that responses with a `OK`
$ surl -XGET localhost:8080/api/health -d'OK'

# mock a POST api that responses a json object
$ surl -XPOST localhost:8080/api/items/2 -d'{"id": 2}'

# mock a POST api that responses the original post payload, using jsonpath (TODO)
$ surl -XPOST -d'$.' localhost:8080/api/items

# extract response from a json object according to a path variable, using jsonpath (TODO)
$ surl -XGET -d'{"id=1": {"id": 1}}' localhost:8080/api/items/:id

# render a response with a path variable (TODO)
$ surl -XGET -d'{"id=1": {"id": 1}}' localhost:8080/api/items/:id

# anything else?
```

## see full arguments

```
surl -h
```