Desde que las plantillas de correo electrónico admiten variables y lógica, es posible crear plantillas
que fallen al renderizar, o a veces fallen al renderizar.

Esto puede ser muy frustrante de diagnosticar y depurar, especialmente si es un problema intermitente, o
si solo ocurre cuando los datos tienen cierta apariencia.

Para ayudar, FastComments Email Templates tiene un par de características:

1. Si la plantilla falla al previsualizar, no se puede guardar. Se mostrará un mensaje de error.
2. Los fallos de renderizado de plantillas se rastrean e informan en la interfaz de administración.

El segundo punto describe fallos de renderizado que ocurren en producción. Es decir, creas una plantilla que se previsualiza
bien, pero que más tarde falla por alguna razón. Por ejemplo, si tenemos esto en nuestra plantilla:

    <% if (comment.commenterEmail.includes('test') { %>

Esto puede fallar a veces si tenemos comentarios anónimos habilitados, ya que el correo electrónico no siempre estará
disponible. Entonces, ¿cómo nos enteramos de eso?

La respuesta es que los errores se muestran en dos lugares. Primero, la propia lista de plantillas
muestra un recuento de errores de renderizado junto a cada plantilla.

Luego, al ver una plantilla podemos ver un recuento, por error, del número de veces que la plantilla
ha fallado al renderizar.

Hay un botón de restablecer ubicado junto a cada error y su recuento, para que podamos reiniciar el contador
después de haber resuelto un problema.

---