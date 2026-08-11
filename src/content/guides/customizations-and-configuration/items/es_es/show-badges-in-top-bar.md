---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará los distintivos de usuario solo en sus comentarios dentro del hilo de comentarios.

Sin embargo, podemos mostrar los distintivos de usuario junto a su nombre encima del formulario de comentarios habilitando esta función en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Casilla de verificación de mostrar distintivos en la barra superior en la página de personalización del widget, colocando los distintivos junto al nombre encima del formulario de comentarios'; title='Opción de Mostrar Distintivos en la Barra Superior' app-screenshot-end]

Esto mostrará los distintivos del usuario junto a su nombre en el área de la barra superior, haciendo que sus logros y estado sean más prominentes cuando estén redactando un comentario.

Tenga en cuenta que esta función debe estar habilitada en la interfaz de personalización del widget para funcionar. Opcionalmente, puede establecer la bandera **showBadgesInTopBar** en false en la configuración de su código para desactivarla selectivamente incluso cuando esté activada a nivel del servidor:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Desactivar Mostrar Distintivos en la Barra Superior'; code-example-end]
---