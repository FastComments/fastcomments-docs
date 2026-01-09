---
[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

El recuento de comentarios que se muestra en la parte superior del widget de comentarios puede mostrar ya sea todos los comentarios "nivel superior", es decir, aquellas respuestas que
son respuestas directamente a la página o al artículo en sí, o puede ser un recuento de **todos** los comentarios anidados.

Por defecto, esto es `true` - es un recuento de lo último - todos los comentarios. En versiones anteriores del widget de comentarios el
valor predeterminado es `false`.

Podemos cambiar el comportamiento, de modo que sea un recuento de **todos** los comentarios anidados estableciendo la bandera **countAll** en true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Si quisiéramos que el recuento reflejara sólo los comentarios de nivel superior, establecemos la bandera en false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Esto actualmente no puede personalizarse sin cambios en el código.

---