---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará las insignias de los usuarios solo en sus comentarios dentro del hilo de comentarios.

Sin embargo, podemos mostrar las insignias de los usuarios junto a su nombre encima del formulario de comentario activando esta opción en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Esto mostrará las insignias del usuario junto a su nombre en el área de la barra superior, haciendo que sus logros y su estado sean más visibles cuando estén redactando un comentario.

Tenga en cuenta que esta función debe estar activada en la interfaz de personalización del widget para que funcione. Opcionalmente, puede establecer la bandera **showBadgesInTopBar** en false en su configuración de código para desactivarla selectivamente incluso cuando esté activada a nivel de servidor:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---