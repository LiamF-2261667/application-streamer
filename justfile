#!/usr/bin/env just --justfile

# Using Just: https://github.com/casey/just?tab=readme-ov-file#installation
set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe","-c"]

export RUST_BACKTRACE := "1"
export RUST_LOG := "info"

# List all of the available commands.
default:
  just --list

# Install any required dependencies.
setup:
	# Upgrade Rust
	rustup update

	# Make sure the WASM target is installed.
	rustup target add wasm32-unknown-unknown

	# Make sure the right components are installed.
	rustup component add rustfmt clippy

	# Install cargo binstall if needed.
	cargo install cargo-binstall

	# Install cargo shear if needed.
	cargo binstall --no-confirm cargo-shear

	# Install cross for cross-compilation to linux.
	cargo install cross --git https://github.com/cross-rs/cross
	rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host

# Build the application streamer for linux
application-streamer:
    cross build --bin application-streamer \
        --target-dir application-streamer/target \
        --target x86_64-unknown-linux-gnu \


application-streamer-native:
    cargo build --bin application-streamer \
        --target-dir application-streamer/target

# Run the relay, web server, and publish bbb.
all:
	npm i && npx concurrently --kill-others --names srv,bbb,web --prefix-colors auto "just relay" "sleep 1 && just bbb" "sleep 2 && just web"

# Run a localhost relay server
relay:
	cargo run --bin moq-relay -- --bind "[::]:4443" --tls-self-sign "localhost:4443" --cluster-node "localhost:4443" --tls-disable-verify

# Run a localhost leaf server, connecting to the relay server
leaf:
	cargo run --bin moq-relay -- --bind "[::]:4444" --tls-self-sign "localhost:4444" --cluster-node "localhost:4444" --cluster-root "localhost:4443" --tls-disable-verify

# Run a cluster of relay servers
cluster:
	npm i && npx concurrently --kill-others --names root,leaf,bbb,web --prefix-colors auto "just relay" "sleep 1 && just leaf" "sleep 2 && just bbb" "sleep 3 && just web"

# Download and stream the Big Buck Bunny video to the localhost relay server
bbb: (download "bbb" "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4") (pub "bbb")

# Download and stream the Big Buck Bunny video to localhost directly
bbb-server: (download "bbb" "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4") (pub-server "bbb")

# Download and stream the inferior Tears of Steel video
tos: (download "tos" "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4") (pub "tos")

# Download and stream AV1 content:
av1: (download "av1" "http://download.opencontent.netflix.com.s3.amazonaws.com/AV1/Sparks/Sparks-5994fps-AV1-10bit-1920x1080-2194kbps.mp4") (pub "av1")

# Download and stream HEVC content:
hevc: (download "hevc" "https://test-videos.co.uk/vids/jellyfish/mp4/h265/1080/Jellyfish_1080_10s_30MB.mp4") (pub "hevc")

# Download the video and convert it to a fragmented MP4 that we can stream
download name url:
	if [ ! -f dev/{{name}}.mp4 ]; then \
		wget {{url}} -O dev/{{name}}.mp4; \
	fi

	if [ ! -f dev/{{name}}.fmp4 ]; then \
		ffmpeg -i dev/{{name}}.mp4 \
			-c copy \
			-f mp4 -movflags cmaf+separate_moof+delay_moov+skip_trailer+frag_every_frame \
			dev/{{name}}.fmp4; \
	fi

# Publish a video using ffmpeg to the localhost relay server
pub name:
	# Pre-build the binary so we don't queue media while compiling.
	cargo build --bin moq-karp

	# Run ffmpeg and pipe the output to moq-karp
	ffmpeg -hide_banner -v quiet \
		-stream_loop -1 -re \
		-i "dev/{{name}}.fmp4" \
		-c copy \
		-f mp4 -movflags cmaf+separate_moof+delay_moov+skip_trailer+frag_every_frame \
		- | cargo run --bin moq-karp -- publish "http://localhost:4443/demo/{{name}}"

# Publish a video using ffmpeg directly from moq-karp to the localhost
pub-server name:
	# Pre-build the binary so we don't queue media while compiling.
	cargo build --bin moq-karp

	# Run ffmpeg and pipe the output to moq-karp
	ffmpeg -hide_banner -v quiet \
		-stream_loop -1 -re \
		-i "dev/{{name}}.fmp4" \
		-c copy \
		-f mp4 \
		-movflags cmaf+separate_moof+delay_moov+skip_trailer+frag_every_frame \
		- | cargo run --bin moq-karp -- --server --bind "[::]:4443" --tls-self-sign "localhost:4443" --tls-disable-verify publish "http://localhost:4443/"

# Run the stream deliverer
run:
	npm i && npm run dev

# Run the CI checks
check:
	cargo check --all-targets
	cargo check -p moq-web --target wasm32-unknown-unknown
	cargo clippy --all-targets -- -D warnings
	cargo clippy -p moq-web --target wasm32-unknown-unknown
	cargo fmt -- --check
	cargo shear # requires: cargo binstall cargo-shear
	npm i && npm run check

# Run any CI tests
test:
	cargo test

# Automatically fix some issues.
fix:
	cargo fix --allow-staged --all-targets --all-features
	cargo clippy --fix --allow-staged --all-targets --all-features
	cargo clippy -p moq-web --target wasm32-unknown-unknown --fix --allow-staged --all-targets --all-features
	cargo fmt --all
	npm i && npm run fix
	cargo shear --fix

# Upgrade any tooling
upgrade:
	rustup upgrade

	# Install cargo-upgrades if needed.
	cargo install cargo-upgrades cargo-edit
	cargo upgrade

	# Update the NPM dependencies
	npm update
	npm outdated

# Build the release NPM package
build:
	npm i && npm run build

# Build and link the NPM package
link:
	npm i && npm run build:dev && npm run build:tsc
	npm link

# Delete any ephemeral build files
clean:
	rm -r dist
