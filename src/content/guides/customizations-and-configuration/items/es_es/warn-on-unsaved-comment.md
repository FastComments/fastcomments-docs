[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Por defecto, si un usuario escribe un comentario y luego actualiza la página, cierra la pestaña o navega fuera antes de enviarlo, el borrador se pierde silenciosamente.

Establecer **warnOnUnsavedComment** a true hace que el navegador pida al usuario que confirme antes de salir de la página mientras cualquier cuadro de comentario, o una edición en curso, aún contiene texto. Una vez que el comentario se envía, el texto se borra, por lo que no se muestra ningún aviso.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Advertir sobre comentario no guardado'; code-example-end]

El aviso utiliza el propio cuadro de diálogo del navegador. Los navegadores modernos muestran su propio texto y ignoran el texto personalizado, por lo que el mensaje no se puede personalizar.

Esta opción carga una pequeña extensión bajo demanda, por lo que no añade nada al widget para los sitios que no la habilitan.