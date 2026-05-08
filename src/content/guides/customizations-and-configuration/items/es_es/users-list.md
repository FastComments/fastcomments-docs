[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments no muestra una lista de usuarios en la página.

Puedes mostrar una lista de personas que están viendo la página en ese momento, junto al widget de comentarios. La lista se actualiza en tiempo real a medida que los usuarios se conectan y se desconectan, y muestra su nombre, avatar y un indicador de presencia en línea.

Hay tres opciones de diseño:

- `1` - Top: una fila horizontal de avatares superpuestos que se muestran encima de los comentarios.
- `2` - Left: una barra lateral con nombres y puntos de estado en línea mostrada a la izquierda del widget.
- `3` - Right: la misma barra lateral mostrada a la derecha del widget.

Activa la función estableciendo la bandera **usersListLocation**:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Por defecto, la lista muestra solo a los usuarios que están actualmente en línea. Para incluir también a las personas que han comentado en la página en el pasado (pero que no la están viendo ahora), establece **usersListIncludeOffline** en true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Los comentaristas anteriores se muestran sin el punto verde de presencia en línea, de modo que queda claro quién está presente en este momento.

Los usuarios con perfiles privados se muestran con un avatar genérico y una etiqueta "Private Profile" para que el recuento siga siendo exacto sin revelar identidades.

Esto también se puede configurar sin código. En la página de personalización del widget, consulta la opción "Users List Location". Cuando la ubicación se establece en cualquier valor distinto de Desactivado, aparece una casilla "Incluir comentaristas anteriores" debajo.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---