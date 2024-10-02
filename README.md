# how to get this shit running

## 1 run docker
docker buildx build --platform linux/arm64 -t my-custom-cross --load .

## 2 compile by using the build for linux bat
## 3 move into a folder:
 - the executable
 - the Cargo.toml file
     - change Cargo.toml to use site-root = "site" instead of "target/site"
 - the target/site folder into just site/
 - the resources folder
