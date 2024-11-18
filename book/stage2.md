# Etapa 2
Bienvenido al segundo nivel para oxidar `ls`!

Aquí necesitaras interpretar los argumentos con los que es ejecutado tu programa y deberas extender el listado original para incluir aquellos archivos ocultos que `ls` no muestra por defecto.

## Cómo pasar el nivel

Debes imprimir en consola la lista de archivos que encuentres en el directorio donde se ejecute. Incluido los **ocultos**!

El tester de este nivel ejecutara tu comando con los argumentos:
* `-a` 
* `--all`

Ambos argumentso deben dar el mismo resultado. Debes listar los archivos del directorio, sin informacion adicional, e incluir aquellos archivos que son ocultos por defecto.

Nuevamente nuestro tester ejecutará la version GNU de `/bin/ls`, con los mismos argumentos, para comparar el standard output. La comparación idéntica será la validacion del nivel.

Exitos!

## Lecturas Recomendadas
Para completar este ejercicio, te recomendamos que te familiarices con:
* [Command Line Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
