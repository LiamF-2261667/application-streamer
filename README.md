The project is split into a few crates:

-   [moq-relay](moq-relay): A server that forwards content from publishers to any interested subscribers. It can optionally be clustered, allowing N servers to transfer between themselves.
-   [stream-deliverer](stream-deliverer): A web application utilizing Rust and WASM. Supports watching an application stream running on port 4443.
-   [moq-transfork](moq-transfork): The underlying network protocol. It can be used by live applications that need real-time and scale, even if they're not media.
-   [moq-karp](moq-karp): The underlying media protocol powered by moq-transfork. It includes a CLI for importing/exporting to other formats, for example integrating with ffmpeg.
-   [moq-native](moq-native): Helpers to configure the native MoQ tools.
-   [mp4-atom](mp4-atom): A library for parsing MP4 files. It can be used to extract the moof/mdat atoms from a file, or to parse the entire file.
-   [application-streamer](application-streamer): The underlying application that should be run inside a Docker container to stream an application.

This is a fork of [moq-rs](https://github.com/kixelated/moq-rs), more information about each component can be found in the original repository.

# Usage
## Requirements
- [Rustup](https://www.rust-lang.org/tools/install)
- [Just](https://github.com/casey/just?tab=readme-ov-file#installation)
- [Node + NPM](https://nodejs.org/)

## Setup
We use `just` to simplify the development process.
Check out the [Justfile](justfile) or run `just` to see the available commands.

Install any other required tools:
```sh
just setup
```

## Development

```sh
# Build the application streamer:
just application-server # for windows
just application-server-native # for linux
```

```sh
# Run the pre-build application streamer:
docker build -t application-streamer -f application-streamer/Dockerfile .
docker run -it --rm --gpus all -p 4443:4443 application-streamer
```

```sh
# Run the stream deliverer:
just stream-deliverer
```

Then, visit [https://localhost:8080](localhost:8080) to watch the application stream.