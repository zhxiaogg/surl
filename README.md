# surl "server URL"

## server directives

```
$ surl server start
$ surl server stop
$ surl server list (TODO)
$ surl server watch (TODO)
```

## mocking examples

```
# mock a GET api that responses with a plain text `OK`
$ surl -XGET localhost:8080/api/health -d'OK'

# mock a POST api that responses a json object and `content-type` header
$ surl -XPOST -H'Content-Type:application/json' localhost:8080/api/items/2 -d'{"id": 2}'

# render response with requested body
$ surl -XPOST -d'{{ json body }}' localhost:8080/api/items

# render response with query parameters
$ surl -XGET -d'{"id": {{params.id}} }' localhost:8080/api/items?id=1

# render response with path variables
$ surl -XGET -d'{"id": {{path.id}} }' localhost:8080/api/items/:id

# render response with header values
$ surl -XGET -d'{"id": {{headers.id}} }' localhost:8080/api/items

# anything else?
```

## full usage
```
surl -h
```

## handlebarsjs templating

### context

The surl uses [handlebars-rust](https://github.com/sunng87/handlebars-rust) to render responses. The provided data/context contains the folowing field for each templating operation:
- body: `Some(Json)`, requested body if any (TOOD: the type of body field should conform to requested content-type)
- params: `Map<String,String>`, query params
- path: `Map<String,String>`, path variables
- headers: `Map<String,Option<String>>`, requested headers

### helpers

The surl provides some built-in handlebars helper utilities:
- json, render a non-primitive object into json string
- timestamp, takes 0 params, returns current time in unix timestamp.
- random|random_int, takes 0 params, returns a 32bit signed integer randomly.