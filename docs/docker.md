build
docker build --no-cache -t app-image -f Dockerfile .
run
docker run -p 8080:8080 --rm --name app app-image

docker images --all

prune
docker image prune --all

rm image
docker rmi -f d343aeb50d18