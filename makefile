
all: basic_static blog_home_page

blog_home_page: output


basic_static: output
	cp index.html style.css output

output:
	mkdir output

clean:
	rm -rf output
