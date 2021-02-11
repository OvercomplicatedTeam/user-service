start: build run
build:
	sudo docker build -t user-service .
run:
	sudo docker run -it user-service
db:
	docker run --rm --name some-postgres -p 5432:5432 --env-file .env -d postgres
