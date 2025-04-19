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

## Install rust
RUN apt-get install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

## Initialize the environment
EXPOSE 4443
WORKDIR /usr/bin/application_streamer

## Copy all the required files to compile appliction-streamer (Or mount the directories)
#COPY ./application-streamer ./application_streamer
#COPY ./moq-async ./moq-async
#COPY ./moq-karp ./moq-karp
#COPY ./moq-native ./moq-native
#COPY ./moq-transfork ./moq-transfork
#COPY ./mp4-atom ./mp4-atom
#COPY ./Cargo.toml ./Cargo.toml
#COPY ./Cargo.lock ./Cargo.lock

## Run the application
CMD ["cargo", "run", "--bin", "application-streamer"]