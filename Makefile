# -- Prepare db
# Notice: `loco_hello_development` where `locl_hello` is the app's name 
start_db:
	docker run --rm -p 5432:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=loco_hello_development -e POSTGRES_PASSWORD="loco" postgres:16 

start_db_using_compose:
	docker compose -f dockerfiles/postgres/docker-compose.yaml up

# reset db:
reset_db:
	cargo loco db reset

# -- Start frontend
# Need to install:
# rustup target add wasm32-unknown-unknown
# cargo install --locked trunk
start_frontend:
	cd frontend && dx serve

# -- Start backend
start_backend:
	cargo loco start 

# Below commands need `cargo install sea-orm-cli`
# -- generate some resource for API
generate_post:
	cargo loco generate scaffold post title:string content:text --api

# -- quick dev 
dev_posts:
	make reset_db && cargo run --example quick_dev_for_posts


# Setup SeaORM entities for Seaography 
# Seaography is a GraphQL framework for building GraphQL resolvers using SeaORM entities.
generate_seaorm:
	sea-orm-cli generate entity -o src/models/_entities -u postgres://loco:loco@localhost:5432/loco_hello_development --seaography