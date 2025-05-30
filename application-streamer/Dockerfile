FROM ubuntu:latest
#FROM nvidia/cuda:12.8.1-devel-ubuntu24.04
LABEL authors="liamf"

# Install xvfb, ffmpeg and xdotool
RUN apt-get update --fix-missing
RUN apt-get install -y xvfb
RUN apt-get install -y ffmpeg
RUN apt-get install -y xdotool

## Install google chrome
RUN apt-get install -y wget
RUN apt-get install -y curl
RUN curl -LO https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt-get install -y ./google-chrome-stable_current_amd64.deb
RUN rm google-chrome-stable_current_amd64.deb

EXPOSE 4443

COPY ./target/x86_64-unknown-linux-gnu/debug /usr/bin/application-streamer

CMD ["usr/bin/application-streamer/application-streamer"]