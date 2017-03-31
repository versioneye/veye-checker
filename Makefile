build_linux_image:
	docker build -f Dockerfile_linux_build --rm=true -t veyebox:latest .
