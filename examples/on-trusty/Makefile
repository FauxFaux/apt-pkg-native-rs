build:
	docker build --network=mope --build-arg http_proxy=http://urika:3142 --tag trusty-dev-vm .

run: build
	docker run --network=mope -v $(shell pwd)/../..:/mnt -it trusty-dev-vm

clean:
	docker rmi trusty-dev-vm

.PHONY: build run
