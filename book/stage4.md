# Etapa 4
Bienvenido al cuarto nivel para oxidar `ls`!

En este nivel ayudaras a tus usuarios a usar tu herramienta! Es importante que siempre que construyas una aplicación para la linea de comandos, que incluyas una breve ayuda de como debe ser usado.

## Cómo pasar el nivel

Debes imprimir en consola la lista un texto de ayuda con la indicacion de que argumentos soportas y como debe ser ejecutado. No te olvides que las pruebas son acumulativas ;) 

El tester correra tu aplicación con el siguiente argumento:
* `--help`: imprime la ayuda de tu herramienta.

El resultado esperado el siguiente texto de ayuda:
```
"A tiny version of ls command

Usage: oxidar-ls [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to list

Options:
  -a, --all     Show hidden files
  -F, --format  Format file names. Appendds a symbol at the end indicating the type of file. Slash ('/') is for directories and at sign ('@') is for symbolic links
  -l, --list    Show the files in a list
  -h, --human   Show the file size in a human-redable format
      --help
```

Exitos!

## Lecturas Recomendadas
Para completar este ejercicio, te recomendamos que te familiarices con:
* [Manejo de Errores](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
* [Clap](https://docs.rs/clap/latest/clap/)
