## On Ubuntu 20.04

### Project setup
install rustup:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
add the `x86_64-unknown-linux-musl` target to rustup:
```sh
rustup target add x86_64-unknown-linux-musl
```
install zip:
```sh
sudo apt install zip
```

### Build
build project using predefined target:
```sh
cargo build --release --target x86_64-unknown-linux-musl
```
Store the built application in a zip:
```sh
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```

### Deploy
To update the function code after the lambda has been created:
```sh
aws lambda update-function-code \
  --zip-file fileb://./rust.zip \
  --function-name FUNCTION_NAME \
  --region REGION
```
***

See this [resource](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/) for more details, including
instructions for building on MacOS.
