start: build run
build:
	sudo docker build -t user-service .
run:
	sudo docker run -it user-service
