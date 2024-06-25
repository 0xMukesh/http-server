# http-server

basic implementation of a HTTP web server using rust

## routes

1. GET `/` - serves as a ping route. responds with `200` status code
2. GET `/echo/{str}` - responds with `200`, along with the query passed in place of `str`
3. GET `/user-agent` - responds with `200`, along with value of `User-Agent` header
4. GET `/files/{file}` - checks for a file in the directory passed in the arguments while running `cargo run`. ex: `cargo run --directory tmp` and GET `/files/abc` -- the server looks for `/tmp/abc`. if found, the server responds with `200` along with contents of the file or else `404`
5. POST `/files/{file}` - creates a new file in the directory passed in the arguments while running `cargo run`. ex: `cargo run --directory tmp` and POST `/files/abc` -- the server creates a new file at `/tmp/abc`. if succesfully executed, the server responds with `201` or else `400`
