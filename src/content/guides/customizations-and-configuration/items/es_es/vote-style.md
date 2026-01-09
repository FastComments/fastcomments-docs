[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Por defecto, FastComments mostrará las opciones de votación como flechas hacia arriba y hacia abajo, permitiendo a los usuarios votar a favor o en contra de un comentario.

Sin embargo, es posible cambiar el estilo de la barra de herramientas de votación. Las opciones actuales son los botones predeterminados Arriba/Abajo, o usar un mecanismo de votación con estilo Corazón.

Usamos la opción **voteStyle** de la siguiente manera:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Recomendamos encarecidamente hacerlo sin código, ya que también habilita las validaciones del lado del servidor. En la página de personalización del widget, consulte la sección "Estilo de Votación".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

La votación también puede desactivarse, vea `Disable Voting` arriba de las opciones de estilo.