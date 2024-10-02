# Introducción

¡Bienvenido a Oxidar! El objetivo de este curso es aprender Rust creando un proyecto desde cero. El curso está dividido en varias etapas donde iremos agregando nuevas funcionalidades al proyecto, cada una introduciendo un nuevo concepto o un desafío con lo que ya se vio hasta el momento.

# Herramientas

## cargo

`cargo` es el gestor de paquetes de Rust. Además de gestionar los paquetes, con `cargo` podemos correr tests, construir el proyecto y ejecutar nuestro programa. Los comandos que más utilizaremos son:

- `cargo run -- <args>`: Corre el proyecto pasando como argumentos `<args>`.
- `cargo test`: Corre los tests.
- `cargo add <nombre_de_dependencia>`: Agrega al `Cargo.toml` la dependencia `<nombre_de_dependencia>`

## crates.io

[crates.io](https://crates.io) es el sitio donde se alojan todos los crates públicos o herramientas de líneas de comando que podemos utilizar para construir nuestro proyecto. Generalmente cuando necesitamos una dependencia la solemos buscar aquí.

En esta página no sólo encontraremos el listado de crates disponibles para utilizar. También encontraremos documentación de los crates (si el autor se molestó en crearla) y su código de fuente.

## clippy

`clippy` es el linter oficial de Rust. Esta herramienta nos sirve para marcar potenciales errores y estilar el código de forma correcta.

## rust-analizer

[rust-analizer](https://rust-analyzer.github.io/) es una implementación del [Languange Server Protocol](https://microsoft.github.io/language-server-protocol/) para Rust. El mismo nos provee auto-completado de código entre otras cosas útiles para hacer nuestra vida un poco más sencilla.

## Visual Studio Code

Visual Studio Code tiene gran soporte para Rust. Se recomienda instalar los siguientes plug-ins:

- `rust-analyzer`


# Corriendo el proyecto

Para probar el proyecto, corre `cargo run`. Después de la compilación deberías ver el mensaje `Hello, world!` en la pantalla.
