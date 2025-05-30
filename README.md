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
- [Docker](https://www.docker.com/get-started)

## Setup
We use `just` to simplify the development process.
Check out the [Justfile](justfile) or run `just` to see the available commands.

Install any other required tools:
```sh
just setup
```

## Development

> [!IMPORTANT]
> Make sure the application streamer is built before running the stream deliverer.
> (The generated binaries are used in the stream deliverer.)

```sh
# Build the application streamer:
just application-streamer # for windows
just application-streamer-native # for linux
```

```sh
# Run the stream deliverer:
just run
```

Then, visit [https://localhost:8080](localhost:8080) to watch the application stream.

## Settings

To change the application that will be streamed, edit the [Dockerfile in application-streamer](application-streamer/Dockerfile).
Change the CMD line to run the desired application, for example:
```dockerfile
CMD ["usr/bin/application-streamer/application-streamer google-chrome --no-sandbox https://www.google.com"]
```
The given application will be run inside the Docker container, and its output will be streamed to the stream deliverer.
