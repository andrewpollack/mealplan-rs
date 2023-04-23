# Installs pre-reqs for running frontend framework.
# Trunk -> WASM web application bundler
install:
	cargo install trunk
	rustup target add wasm32-unknown-unknown

run-frontend:
	cd mealplan_website
	trunk serve --open
