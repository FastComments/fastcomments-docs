El script de cada extensión se obtiene e invoca antes de que el widget de comentarios comience a obtener el primer conjunto de comentarios y a renderizar la interfaz de usuario.

En la carga inicial, los siguientes datos se anexarán al objeto de la extensión:

- `config` - A reference to the `config` object.
- `translations` - A reference to the `translations` object.
- `commentsById` - A reference to all comments by id.
- `root` - A reference to the root DOM node.

Las extensiones deben sobrescribir las funciones deseadas, que el widget de comentarios invocará en los momentos apropiados.