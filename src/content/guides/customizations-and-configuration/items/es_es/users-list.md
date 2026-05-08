[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments no muestra una lista de usuarios en la página.

Puedes mostrar una lista de personas que están viendo la página actualmente, junto al widget de comentarios. La lista se actualiza en tiempo real a medida que los usuarios entran y salen, y muestra su nombre, avatar e indicador de conexión.

Hay tres opciones de diseño:

- `1` - Top: una fila horizontal de avatares superpuestos renderizada encima de los comentarios.
- `2` - Left: una barra lateral con nombres y puntos de estado en línea renderizada a la izquierda del widget.
- `3` - Right: la misma barra lateral renderizada a la derecha del widget.

Configura la bandera **usersListLocation** para habilitar la función:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Por defecto, la lista muestra solo a los usuarios que están en línea actualmente. Para incluir también a las personas que han comentado en la página en el pasado (pero que no la están viendo ahora), establece **usersListIncludeOffline** en true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Los comentaristas anteriores se muestran sin el punto verde de conexión para que quede claro quién está presente en este momento.

Los usuarios con perfiles privados se muestran con un avatar genérico y una etiqueta "Perfil privado" para que el conteo siga siendo exacto sin revelar identidades.

Esto también se puede configurar sin código. En la página de personalización del widget, consulta la opción "Ubicación de la lista de usuarios". Cuando la ubicación se establece en cualquier valor distinto de Off, aparece debajo una casilla "Incluir comentaristas anteriores".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Con más de 500 usuarios en línea, la lista puede tener hasta 30 segundos de retraso.

---