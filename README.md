### Run manually
```
$ docker run -p 3001:8080 -e DATABASE_URL=postgres://rustuser:rust@host.docker.internal/leasing docker-rust-image
```

### Run with Docker compose

```
$ docker compose up --build
```

### Start from scratch
```
# Stop and Remove Current Containers:
$ docker-compose down

# Remove Orphaned Volumes (if you want to reset database or persistent data):
$ docker-compose down --volumes

# Rebuild and Start Services:
$ docker-compose up --build

# or for a background process
$docker-compose up --build -d
```


# JUST FORCE PUSH
