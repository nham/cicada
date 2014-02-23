all:
	rustc -L lib -C link-args="-lGL -lglfw" src/glfw_test.rs
