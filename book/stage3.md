# Etapa 3
Bienvenido al tercer nivel para oxidar `ls`!

Aquí necesitaras interpretar aún más argumentos con y deberas empezar a dar formato  al listado de archivos. Te pediremos que incluyas datos adicionales sobre sus atributos.

## Cómo pasar el nivel

Debes imprimir en consola la lista de archivos que encuentres en el directorio donde se ejecute. No olvides incluir los **ocultos**, si el argumento es enviado. Las pruebas son acumulativas ;) 

Los nuevos argumentos que te enviaremos son los siguientes:
* `-l`: lista los archivos en formato largo.
* `-F`: incluye in indicador de directorio `/` al nombre del archivo
* `-h`: indica el tamaño del archivo en un formato humano.

Para ver un ejemplo completo, te recomendamos ejecutar el commando completo de GNU `/bin/ls -alFh`. 
```
drwxrwxrwt  14 root            wheel   448B Nov 18 09:36 ./
drwxr-xr-x   6 root            wheel   192B Nov 18 08:50 ../
drwx------   3 hernangonzalez  wheel    96B Nov 18 08:56 TemporaryDirectory.FzPK1P/
drwx------   3 hernangonzalez  wheel    96B Nov 18 08:56 TemporaryDirectory.JwxVCQ/
drwx------   3 hernangonzalez  wheel    96B Nov 18 08:56 TemporaryDirectory.M4jDdl/
drwx------   3 hernangonzalez  wheel    96B Nov 18 08:56 TemporaryDirectory.cF328d/
drwx------   3 hernangonzalez  wheel    96B Nov 18 09:36 TemporaryDirectory.fBLE6X/
drwx------   3 hernangonzalez  wheel    96B Nov 18 08:56 TemporaryDirectory.saTlk1/
drwx------   3 hernangonzalez  wheel    96B Nov 18 09:36 TemporaryDirectory.xHIr98/
drwx------   3 hernangonzalez  wheel    96B Nov 18 08:56 TemporaryDirectory.yheAih/
drwx------   3 hernangonzalez  wheel    96B Nov 18 08:50 com.apple.launchd.B95BeBAprb/
-rw-r--r--   1 hernangonzalez  wheel     0B Nov 18 08:52 hisory.log
drwxr-xr-x   2 root            wheel    64B Nov 18 08:50 powerlog/
drwx------   2 root            wheel    64B Nov 18 08:50 tmp-mount-iN7AQ3/
```

_No te preocupes, no evaluaremos todas las columnas en este ejercicio. Solo aquellas que fueron pedidas en el argumento que te indicamos._

Exitos!

## Lecturas Recomendadas
Para completar este ejercicio, te recomendamos que te familiarices con:
* [Arreglos en Rust](https://doc.rust-lang.org/book/ch08-01-vectors.html)
* [Slice](https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type)
* [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
