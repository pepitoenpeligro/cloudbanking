# Estudio de Viabilidad

Con las tecnologías propuestas, quiero estudiar la viabilidad del proyecto. No quiero usar tecnologías inestables o que generen problemas a la larga. Tampoco quiero tecnologías incompatibles entre sí.

Para solucionar este problema, he creado un microservicio minimal siguiendo el patrón __API REST__ en Rust, que se puede encontrar en ![music_store](https://github.com/pepitoenpeligro/music_store)


## Lenguaje

He elegido Rust. Es eficiente, genera código sin dependencias, soporta la concurrencia, es tipado (te obliga a pensar en el tipo de fondo), soporta múltiples paradigmas (orientado a objetos, orientado a eventos) y es similar a C++.

He valorado la alternativa Scala, pero frente a Rust pierde en:

* Usa la máquina virtual de java. Es excesivamente lento para proyectos minimales. Puedo tener problemas en el futuro con despliegues y presupuestos.
* La configuración de librerías es un caos. Versiones incompatibles entre sí.
* Como desarrollador te fuerza a la programación funcional aunque ofrezca la posibilida de programar orientado a objetos.


### Frameworks y librerías de Rust 

En esta sección explico qué frameworks y librerías uso y donde.

1. __warp__. Es un framework para crear un servidor web, que permite extracción de parámetros en rutas, responder a rutas, extracción de parámetros de cabecera, serialización y deserialización de cadenas, websockets, cuerpos JSON, compresión Gzip, comunicación asíncrona, HTTP1 y HTTP2. Se usa dentro de `main.rs` y funciona.
2. __serde__. Es un framework para serializar y deserializar structuras de datos de forma eficiente y generalizada. Permite trabajar con JSON, YAML, TOML, BSON, URL, y un sin fin de formatos más. Se usa dentro del modelo de datos `user/model.rs` y funciona.
3. __parkinglot__. Es una libreria que proporciona implementaciones de mecanismos de concurrencia como Cerrojos y mutex a bajo nivel y de forma recursiva y rápida. Se usa dentro de `user/model.rs` para el acceso a la lista de canciones.


### Contenedores

Necesito poder introducir mi aplicación en un contenedor para luego poder establecer una orquestación adecuada de mis microservicios.

Las alternativas eran:

1. Docker
2. LXC

Docker (contenedor de aplicación) ofrece un desarrollo portable entre máquinas, versionado y librerias reusables, así como reusabilidad entre componentes, mientras que `LXC` (contenedor de sistema) ofrece prestaciones geniales en ambientes UNIX, no posee facilidades como la integración continua o el sistema de orquestado que ofrece `Docker`. Por tanto elegimos `Docker`.

En el archivo `Dockerfile` del repositorio [music_store](https://github.com/pepitoenpeligro/music_store) se puede ver lo sencillo que es configurar un contenedor de nuestra aplicación. Se detalla cómo usar dicho `Dockerfile` dentro del `Readme.md` de dicho repositorio.



### Bases de Datos

Teniendo en cuenta que nuestra aplicación está pensada para soportar un gran volumen de conexiones, las bases de datos no relacionales (mysql, maridb, postgresql) quedan descartados.

Por tanto, entre las alternativas nosql más usadas podríamos valorar entre `MongoDB`y `Redis`.

1. MongoDB trabaja con documentos JSON de forma nativa, permite una estructura flexible y es fácilmente escalable horizontalmente.
2. Redis trabaja con pares clave-valor y no posee un lenguaje de consulta, pero al ejecutarse en memoria, nos permite trabajar con muy poca latencia y mayor velocidad.

Por tanto, propongo MongoDB como base de datos principal, y Redis como sistema caché.

En el proyecto del repositorio, no he usado MongoDB, pero sí lo he usado en proyectos anteriores como en:

1. [Test PER](https://testalv.herokuapp.com)
2. [VCSserver](https://github.com/pepitoenpeligro/VCSserver)

Por lo que conozco su uso, integración y despliegue.


### Sistema de Control de Versiones

El sistema de control de versiones por defecto de `Rust` es `git`. He usado `git` con anterioridad y también sus competidores como `subversion` o `mercurial`. Veo mayor alcance del proyecto como programador usando `git`.






## Referencias
* [warp](https://github.com/seanmonstar/warp)
* [serde](https://github.com/serde-rs/serde)
* [parking_lot](https://github.com/Amanieu/parking_lot)