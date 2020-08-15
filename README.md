## Mount into named volume
Copy this folder into a named volume to get started - [Source](https://stackoverflow.com/a/37469637/2727983)
```
docker run -v cos700-project:/data --name helper busybox true
docker cp . helper:/data
docker rm helper
```