# Notice: `loco_hello_development` where `locl_hello` is the app's name 
start_db:
	docker run -d -p 5432:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=loco_hello_development -e POSTGRES_PASSWORD="loco" postgres:16

# -- Start frontend
start_frontend:
	cd frontend && trunk serve --watch src

# -- Start backend
start_backend:
	cargo loco start


# Below commands need `cargo install sea-orm-cli`
# -- generate some resource for API
generate_post:
	cargo loco generate scaffold post title:string content:text --api