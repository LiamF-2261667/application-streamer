#FROM ubuntu:latest
FROM nvidia/cuda:12.8.1-devel-ubuntu24.04
LABEL authors="liamf"

# Install xvfb as X-Server, ffmpeg & kate
RUN apt-get update
RUN apt-get install -y xvfb
RUN apt-get install -y ffmpeg
RUN apt-get install -y kate
RUN apt-get install -y xdotool
RUN apt-get install -y wget
#RUN apt-get install -y alsa
#RUN apt-get install -y dbus-x11
#RUN apt-get install -y pulseaudio
RUN wget https://github.com/brendangregg/perf-tools/archive/refs/heads/master.zip

## Install google chrome
RUN apt-get install -y curl
RUN curl -LO https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt-get install -y ./google-chrome-stable_current_amd64.deb
RUN rm google-chrome-stable_current_amd64.deb

EXPOSE 4443

COPY ./target/x86_64-unknown-linux-gnu/debug /usr/bin/application-streamer

CMD ["usr/bin/application-streamer/application-streamer"]