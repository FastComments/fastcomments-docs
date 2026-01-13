[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments tendrá habilitados los comentarios en vivo.

Esto significa que cada visualizador del hilo de comentarios debería ver el mismo contenido.

Por ejemplo, si se añade un comentario, ese comentario se mostrará. Si se edita o elimina un comentario,
entonces esos comentarios se editarán o eliminarán para todos los visualizadores del hilo. Lo mismo con los votos y todas las acciones de moderación.

Sin embargo, podemos desactivar esto:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, consulte la sección "Desactivar comentarios en vivo".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---