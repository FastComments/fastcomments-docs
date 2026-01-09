[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments no rastrea quién vio cada comentario ni proporciona estadísticas al respecto.

Sin embargo, podemos habilitar esta función, y entonces el sistema empezará a registrar cuando cada usuario se desplace hasta un comentario.

Cuando esto ocurre, se incrementará un contador junto a un icono de ojo que se muestra en cada comentario. El contador se actualiza en tiempo real y se abrevia según la configuración regional del usuario.

Podemos habilitar esto estableciendo la bandera **enableViewCounts** en true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Registramos el id de usuario* que vio el comentario, de modo que si vuelves a ver el comentario no se incrementará. Si vuelves a ver el comentario
después de dos años, el contador se incrementará de nuevo.

- *Nota: o el id de sesión anónima, o la IP del usuario como un valor hash.