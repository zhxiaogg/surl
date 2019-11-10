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
# mock a GET api that responses with a plain `OK`
$ surl -XGET localhost:8080/api/health -d'OK'

# mock a POST api that responses a json object and `content-type` header
$ surl -XPOST -H'Content-Type:application/json' localhost:8080/api/items/2 -d'{"id": 2}'

# mock a POST api that renders a response with the original post body.
$ surl -XPOST -d'{{ json body }}' localhost:8080/api/items

# mock a GET api that renders a response with the query parameters.
$ surl -XGET -d'{"id": {{params.id}} }' localhost:8080/api/items?id=1

# render a response with a path variable (TODO)
$ surl -XGET -d'{"id": {{path.id}} }' localhost:8080/api/items/:id

# render a response with a header value (TODO)
$ surl -XGET -d'{"id": {{headers.id}} }' localhost:8080/api/items/:id

# anything else?
```

## usage

see full help:
```
surl -h
```

## handlebarsjs templating

### context

The surl uses [handlebars-rust](https://github.com/sunng87/handlebars-rust) to render responses. The provided data/context contains the folowing field for each templating operation:
- body: Some(Json), requested body if any (TOOD: the type of body field should conform to request content-type)
- params: `Map<String,String>`, query params
- path: `Map<String,String>`, path variables (TODO: implement this)
- headers: `Map<String,String>`, requested headers (TODO: implement this)

### helpers

The surl provides some built-in handlebars helper utilities:
- json, render a non-primitive object into json string
- TODO