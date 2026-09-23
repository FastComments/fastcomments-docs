[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments no rastrea quién vio cada comentario ni proporciona estadísticas al respecto.

Sin embargo, podemos habilitar esta función, y el sistema comenzará a rastrear a medida que cada usuario se desplace a un comentario.

Cuando esto ocurre, un contador junto a un ícono de ojo que se muestra en cada comentario se incrementará. El contador se actualiza en tiempo real y se abrevia según la configuración regional del usuario.

Podemos habilitar esto configurando la bandera **enableViewCounts** a true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Habilitando el recuento de vistas de comentarios'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Página de personalización del widget con la casilla de recuento de vistas marcada, de modo que cada comentario muestra un ícono de ojo y un recuento'; title='Habilitando el recuento de vistas de comentarios' app-screenshot-end]

Rastreamos el id de usuario* que vio el comentario durante una semana, de modo que si vuelves a ver el comentario dentro de esa semana no se incrementará. Si vuelves a ver el comentario después de que haya pasado la semana, el contador se incrementará nuevamente.

- *Nota: o el id de sesión anónima, o la IP del usuario como un valor hash.