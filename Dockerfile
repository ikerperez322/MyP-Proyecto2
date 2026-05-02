FROM rust:latest

RUN apt-get update && apt-get install -y \
    libgtk-4-dev \
    libgl1-mesa-dev \
    libgl1-mesa-dri \
    libglib2.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    librsvg2-dev \
    pkg-config \
    alsa-utils \
    libasound2-dev \
    libx11-dev \
    xkb-data \
    libxkbcommon-dev \
    libssl-dev \
    sqlite3 \
    libsqlite3-dev \
    mesa-utils \
    x11-utils \
    dbus-x11 \

    libwayland-dev \
    wayland-protocols \
    libwayland-cursor0 \
    libwayland-egl1 \

    weston \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

RUN mkdir -p bd && chmod 777 bd 

COPY Cargo.toml Cargo.lock ./
COPY modelo modelo
COPY controlador controlador
COPY vista vista
COPY aplicacion aplicacion

# ENV GDK_BACKEND=x11
# ENV LIBGL_ALWAYS_SOFTWARE=1
# ENV GSK_RENDERER=gl

RUN cargo build --release

CMD ["./target/release/aplicacion"]





