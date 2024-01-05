# create make copy of makefile
# copy ./line_sender and ./server_disk to ../performance_notify_gitlab and including files in both directories

# make copy
make copy:
	cp -r ./line_sender ../performance_notify_gitlab
	cp -r ./server_disk ../performance_notify_gitlab
	cp .gitignore ../performance_notify_gitlab