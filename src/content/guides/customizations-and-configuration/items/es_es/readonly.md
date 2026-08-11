[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Se puede bloquear la capacidad de comentar para que no se puedan dejar nuevos comentarios o votos estableciendo la bandera readonly en true.

Los comentarios tampoco podrán ser editados o eliminados.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Haciendo el hilo de comentarios de solo lectura'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, para un dominio completo o una página:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Configuración de evitar nuevas respuestas en la página de personalización del widget, que bloquea un hilo para un dominio o página'; title='Haciendo el hilo de comentarios de solo lectura' app-screenshot-end]

## ¡Actualización!

A partir de noviembre de 2022, los hilos pueden bloquearse o desbloquearse **en vivo** por administradores y moderadores mediante el menú de tres puntos encima del área de respuesta.

Esto evitará nuevos comentarios, mientras sigue permitiendo votar y permite a los usuarios eliminar sus comentarios si lo desean, mientras que `readonly` no permite estas cosas. 

Esto corresponde al campo `isClosed` en la API `Page`.