[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments tendrá los comentarios en vivo habilitados.

Esto significa que cada espectador del hilo de comentarios debería ver el mismo contenido.

Por ejemplo, si se agrega un comentario, ese comentario debería mostrarse. Si un comentario se edita o elimina,
entonces esos comentarios se editarán o eliminarán para todos los espectadores del hilo. Lo mismo ocurre con los votos y todas las acciones de moderación.

Sin embargo, podemos desactivar esto:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, vea la sección "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Sección de Desactivar Comentarios en Vivo de la página de personalización del widget, desactivando las actualizaciones en tiempo real del hilo'; title='Desactivar Comentarios en Vivo' app-screenshot-end]