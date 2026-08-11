[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Por defecto, FastComments mostrará las opciones de votación como flechas hacia arriba y hacia abajo, permitiendo a los usuarios votar un comentario hacia arriba o hacia abajo.

Sin embargo, es posible cambiar el estilo de la barra de votación. Las opciones actuales son los botones predeterminados de Arriba/Abajo, o usar un mecanismo de votación con estilo de Corazón.

Usamos la bandera **voteStyle** de la siguiente manera:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Activar botón de corazón'; code-example-end]

Recomendamos encarecidamente que lo hagas sin código, ya que también habilita validaciones del lado del servidor. En la página de personalización del widget, consulta la sección "Estilo de voto".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Configuración de estilo de voto en la página de personalización del widget, ofreciendo flechas arriba y abajo o voto con corazón'; title='Cambiar estilo de votación' app-screenshot-end]

La votación también puede desactivarse, consulta `Disable Voting` arriba de las opciones de estilo.