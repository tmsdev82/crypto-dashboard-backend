# Warp backend with websockets

This project is a basic foundation for a websocket enabled backend using the warp library.

## Using

The backend can be started with the cargo command:

```bash
cargo run
```

Logging is on `INFO` level by default. To show `DEBUG` level logging start with the following command:

```bash
RUST_LOG=debug cargo run
```

A health check can be done by doing a HTTP GET request on the `health` endpoint:

```bash
curl -X GET 'http://0.0.0.0:8000/health'
```

Register a client using the following command in a terminal:

```bash
curl -X POST 'http://0.0.0.0:8000/register' -H 'Content-Type: application/json' -d '{ "client_id": 1 }'
```

The return value will be a websocket uri that can be connect to with a websocket client, for example websocat:

```bash
websocat -t ws://127.0.0.1:8000/ws/<uuid>
```

Messages can be sent to connected clients using an endpoint for example:

```bash
curl -X POST 'http://0.0.0.0:8000/publish' -H 'Content-Type: application/json' -d '{ "channel": "trades", "message": "trading is going well" }'
```