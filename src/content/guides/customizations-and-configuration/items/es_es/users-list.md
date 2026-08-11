[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments no muestra una lista de usuarios en la página.

Puedes renderizar una lista de personas que están viendo la página en ese momento, junto al widget de comentarios. La lista se actualiza en tiempo real a medida que los usuarios se unen y salen, y muestra su nombre, avatar y un indicador de conexión.

Hay tres opciones de diseño:

- `1` - Superior: una fila horizontal de avatares superpuestos renderizada encima de los comentarios.
- `2` - Izquierda: una barra lateral con nombres y puntos en línea renderizada a la izquierda del widget.
- `3` - Derecha: la misma barra lateral renderizada a la derecha del widget.

Establece la bandera **usersListLocation** para habilitar la función:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Mostrar lista de usuarios a la derecha'; code-example-end]

Por defecto, la lista muestra solo a los usuarios que están en línea. Para incluir también a personas que han comentado en la página en el pasado (pero que no la están viendo actualmente), establece **usersListIncludeOffline** a true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Incluir comentaristas anteriores'; code-example-end]

Los comentaristas anteriores se renderizan sin el punto verde de en línea, de modo que quede claro quién está presente ahora.

Los usuarios con perfiles privados se muestran con un avatar genérico y una etiqueta "Perfil privado" para que el recuento siga siendo preciso sin revelar identidades.

Esto también se puede configurar sin código. En la página de personalización del widget, consulta la opción "Ubicación de la lista de usuarios". Cuando la ubicación se establece en cualquier valor distinto de Off, aparece una casilla de verificación "Incluir comentaristas anteriores" debajo.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Ubicación de la lista de usuarios establecida a la derecha, con la casilla de verificación Incluir comentaristas anteriores mostrada debajo'; title='Configuración de la lista de usuarios'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Con más de 500 usuarios en vivo, la lista puede estar desactualizada hasta 30 segundos.