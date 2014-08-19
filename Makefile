#All this make file does is create tags in the project directory

all:
	ctags -R -f .
