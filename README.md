# MyP-Proyecto2

Reproductor y base de datps musical correspondiente al segundo proyecto de la materia de Modelado y Programación.

Este proyecto usa sqlite para manejar la base de datos, gtk4 para la interfaz gráfica y el crate Rodio para la reproducción de archivos mp3, por estos dos últimos la ejecución en Docker puede causar problemas.

---

## Ejecución

### Uso con Docker:

Antes que nada se debe dar autorización a Docker de conectarse al servidor X11:

``` bash
	xhost +local:docker
```

#### Construir el servidor:

```bash
	docker build -t reproductor-musica .
```

#### Ejecutar la aplicación:

Si se está utilizando Wayland el comando es el siguiente:

```bash
	docker run -it --rm \
  -e XDG_RUNTIME_DIR=/tmp/runtime-dir \
  -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
  -v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:/tmp/runtime-dir/$WAYLAND_DISPLAY \
  -v /tmp/runtime-dir:/tmp/runtime-dir \
  -e GDK_BACKEND=wayland \
  --user=$(id -u):$(id -g) \
  reproductor-musica
```

Por otro lado, si se utiliza X11 primero se necesitan descomentar las siguientes líneas del Dockerfile:

``` Dockerfile
	# ENV GDK_BACKEND=x11
	# ENV LIBGL_ALWAYS_SOFTWARE=1
	# ENV GSK_RENDERER=gl
```

Posteriormente se ejecuta el siguiente comando:

``` bash
	docker run -it --rm \
    -e DISPLAY=$DISPLAY \
    -e GDK_BACKEND=x11 \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v ~/Musica:/root/Musica \
    -v $(pwd)/data:/root/.local/share/reproductor \
    --device /dev/snd \
    reproductor-musica
```

---

### Uso con Cargo:

Se recomienda usar esta opción ya que con Docker la aplicación no siempre se muestra correctamente (en particular con Wayland), además de que la reproducción de Música no siempre funciona de la forma esperada.

Para ejecutar la aplicación usando Cargo, basta con poner en la raíz del proyecto:

``` bash
	cargo new
```

No hace falta agregar ninguna dependencia ya que todas están declaradas en los archivos `.toml` del proyecto por lo que Cargo se encargará de instalarlas automáticamente.

---
