[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Por defecto, la función de comentarios en vivo está habilitada. Esto significa que si se añaden, eliminan, editan o fijan comentarios, los cambios deberían aparecer
a todos los usuarios que estén viendo el hilo de comentarios al mismo tiempo.

Sin embargo, por defecto esos nuevos comentarios aparecerán bajo un botón mostrado dinámicamente con un texto similar a "Mostrar 2 comentarios nuevos".

Si los nuevos comentarios son respuestas directamente a la página, el botón se mostrará en la parte superior del hilo de comentarios. Si son respuestas a un comentario en particular,
el botón se mostrará debajo de ese comentario.

Esto es para evitar que el tamaño de la página cambie constantemente para el usuario, lo que podría causar frustración al intentar agarrar la barra de desplazamiento.

Para algunos casos de uso, como subastas en vivo o eventos en línea, este no es el comportamiento deseado: es posible que quiera que el widget de comentarios sea
más como una caja de "chat" donde los nuevos comentarios se "muestren de inmediato".

Por ello, el nombre de la bandera que habilita esa función: **showLiveRightAway**.

Podemos activarlo de la siguiente manera:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---