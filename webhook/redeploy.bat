docker stop cd-test
docker rm cd-test
docker run -d -p 8000:8000 --name cd-test lucasbrunner/cd-test:%1