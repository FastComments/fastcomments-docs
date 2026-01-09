[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Se puede bloquear la funcionalidad de comentarios para que no se puedan dejar nuevos comentarios ni votos, estableciendo la bandera readonly en true.

Los comentarios tampoco podrán editarse ni eliminarse.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, para todo un dominio o una página:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## ¡Actualización!

A partir de noviembre de 2022, los hilos pueden bloquearse o desbloquearse **en vivo** por administradores y moderadores mediante el menú de tres puntos sobre el área de respuesta.

Esto impedirá nuevos comentarios, a la vez que permitirá la votación y que los usuarios eliminen sus comentarios si lo desean, mientras que `readonly` no permite estas acciones. 

Esto corresponde al campo `isClosed` en la API `Page`.

---