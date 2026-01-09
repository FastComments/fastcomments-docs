[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Por defecto, los nuevos comentarios en vivo aparecen en la parte superior de la lista de comentarios a medida que se publican en tiempo real.

Cuando esta opción está habilitada, los nuevos comentarios en vivo se añadirán en la parte inferior de la lista en su lugar. Esto afecta a cómo aparecen los comentarios cuando se publican en directo mientras los usuarios están viendo el hilo de comentarios.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Con esta configuración habilitada:
- Los nuevos comentarios en vivo publicados por otros usuarios aparecerán en la parte inferior de la lista de comentarios
- Los usuarios verán aparecer nuevos comentarios por debajo de los comentarios existentes en tiempo real
- Esto solo afecta a las actualizaciones de comentarios en vivo - no a la carga inicial de la página
- Esto puede ayudar a mantener el flujo de lectura cuando los usuarios siguen una discusión

Tenga en cuenta que esta configuración solo afecta al lugar donde se colocan los nuevos comentarios en vivo cuando llegan en tiempo real. No afecta al orden de clasificación inicial cuando se carga la página.
---