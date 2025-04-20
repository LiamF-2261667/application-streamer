FROM ubuntu:latest
LABEL authors="liamf"

# Set the environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install xvfb as X-Server, ffmpeg, xdotool
RUN apt-get update
RUN apt-get install -y xvfb
RUN apt-get install -y ffmpeg
RUN apt-get install -y xdotool
#RUN apt-get install -y alsa
#RUN apt-get install -y dbus-x11
#RUN apt-get install -y pulseaudio

## Install google chrome
RUN apt-get install -y curl
RUN curl -LO https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt-get install -y ./google-chrome-stable_current_amd64.deb
RUN rm google-chrome-stable_current_amd64.deb

### Install rust
RUN apt-get install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

## Initialize the environment
EXPOSE 4443
WORKDIR /application-streamer

## Copy all the required files to compile appliction-streamer (Or mount the directories)
#COPY ./application-streamer ./application-streamer
#COPY ./moq-async ./moq-async
#COPY ./moq-karp ./moq-karp
#COPY ./moq-native ./moq-native
#COPY ./moq-transfork ./moq-transfork
#COPY ./mp4-atom ./mp4-atom
#COPY ./Cargo-dev.toml ./Cargo.toml
#COPY ./Cargo.lock ./Cargo.lock
#COPY out/application-streamer/x86_64-unknown-linux-gnu/debug ./

RUN rustup update
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rustfmt clippy
RUN cargo install cargo-binstall
RUN cargo binstall --no-confirm cargo-shear

## Run the application
CMD ["cargo", "run", "--bin", "application-streamer"]
#CMD ["./application-streamer"]