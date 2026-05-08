[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments no muestra una lista de usuarios en la página.

Puedes mostrar una lista de personas que están viendo la página en ese momento, junto al widget de comentarios. La lista se actualiza en tiempo real a medida que los usuarios se unen y se van, y muestra su nombre, avatar e un indicador de conexión.

Hay tres opciones de diseño:

- `1` - Top: una fila horizontal de avatares solapados renderizados encima de los comentarios.
- `2` - Left: una barra lateral con nombres y puntos de conexión renderizada a la izquierda del widget.
- `3` - Right: la misma barra lateral renderizada a la derecha del widget.

Establece la bandera **usersListLocation** para habilitar la función:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Mostrar la lista de usuarios a la derecha'; code-example-end]

Por defecto la lista muestra solo a los usuarios que están en línea actualmente. Para incluir también a las personas que han comentado en la página en el pasado (pero que no la están viendo ahora), establece **usersListIncludeOffline** a true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Incluir comentaristas anteriores'; code-example-end]

Los comentaristas anteriores se muestran sin el punto verde de conexión para que quede claro quién está presente en este momento.

Los usuarios con perfiles privados se muestran con un avatar genérico y una etiqueta "Perfil privado" para que el recuento siga siendo preciso sin revelar identidades.

Esto también puede configurarse sin código. En la página de personalización del widget, consulta la opción "Ubicación de la lista de usuarios":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Ubicación de la lista de usuarios' app-screenshot-end]

Cuando la ubicación se establece en cualquier valor distinto de Off, se muestra la casilla "Incluir comentaristas anteriores" debajo:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Incluir comentaristas anteriores' app-screenshot-end]