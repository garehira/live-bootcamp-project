// recreate tables
sqlx migrate run
docker ps
pgAmin install via snap, search for pgAmind4 in laucher

docker run --name ps-db -e POSTGRES_PASSWORD=POST123 -p 5434:5432 -d postgres:15.2-alpine

sudo systemctl stop postgresql

run the DB standalone \

```shell

docker start ps-db
```

Stop standalone database

```shell

docker stop ps-db
```
