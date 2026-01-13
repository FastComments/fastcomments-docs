[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments renderizará el cuadro de entrada de comentarios y el hilo de comentarios al mismo tiempo. Para ahorrar espacio vertical,
también ocultará cualquier otro campo requerido hasta que se interactúe con el widget.

Sin embargo, el widget de comentarios puede ocultarse detrás de un botón, por ejemplo:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

El botón utiliza texto traducido distinto según si los comentarios están visibles o no. Si los comentarios están ocultos, usa `translations.SHOW_COMMENTS_BUTTON_TEXT`. Si los comentarios están visibles, usa `translations.HIDE_COMMENTS_BUTTON_TEXT`. Las traducciones pueden contener el texto `[count]` que será reemplazado por el conteo localizado.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Esto está diseñado para reemplazar la configuración `hideCommentsUnderCountTextFormat`.

El conteo se actualiza en vivo con el hilo de comentarios. El botón no se muestra si no hay comentarios.

Esto se puede habilitar sin código creando una regla de personalización y activando "Hacer clic para mostrar comentarios":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]