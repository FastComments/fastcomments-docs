[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Por defecto, los comentarios en vivo están habilitados. Esto significa que si se añaden, eliminan, editan o fijan comentarios, los cambios deberían aparecer
para todos los usuarios que están viendo el hilo de comentarios al mismo tiempo.

Sin embargo, por defecto esos nuevos comentarios aparecerán bajo un botón mostrado dinámicamente con un texto similar a "Mostrar 2 Comentarios Nuevos".

Si los nuevos comentarios son respuestas directamente a la página, el botón se mostrará en la parte superior del hilo de comentarios. Si son respuestas a un comentario en particular,
el botón se mostrará bajo ese comentario.

Esto es para evitar que el tamaño de la página cambie constantemente para el usuario, lo que podría causar frustración al intentar agarrar la barra de desplazamiento.

Para algunos casos de uso, como pujas en vivo o eventos en línea, este no es el comportamiento deseado; puede que quieras que el widget de comentarios sea
más como una caja de "chat" donde los nuevos comentarios "se muestren de inmediato".

De ahí el nombre de la bandera que habilita esa función: **showLiveRightAway**.

Podemos activarla de la siguiente manera:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Mostrar comentarios en vivo de inmediato'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Configuración de colapso de comentarios en vivo activada para que los nuevos comentarios aparezcan instantáneamente en lugar de detrás de un botón'; title='Mostrar comentarios en vivo de inmediato' app-screenshot-end]