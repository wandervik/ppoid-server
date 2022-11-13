# ppoid-server

## About
`ppoid-server` listens localhost on port 9090 and reads/writes to `score_board` dynamoDB table. It interacts with `game_index` index that sorts all scores.

Server reactes to 2 types of requests:
- GET /score: returns json list of top 10 scores
- POST /score/{player}/{score}: stores your score in database


## Configure
Folow [this manual](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/getting-started.html) to set up access for your aws acount

## Build
```shell
cargo install --path [install_path]
```

## Run
Just execute
```shell
./[install_path]/ppoid-server
```
## Test
To test if server is configured and running properly run following command:
```shell
curl localhost:9090/score
```