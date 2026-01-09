[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Por defecto, FastComments ordenará los comentarios por la dirección de ordenación "Most Relevant".

La ordenación "Most Relevant" tiene en cuenta la hora en que se dejó el comentario y el número de votos para el ordenamiento.

El usuario puede cambiar la dirección de ordenación a "Más antiguos primero" o "Más recientes primero" en la interfaz del widget de comentarios.

However, we can change the default to be any of the three. For example if you wanted to show the oldest comments first:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Asignamos el valor de **defaultSortDirection** a "OF" para establecer la dirección a "OF".

For the newest-first sort direction, we would do the following:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Los valores válidos para **defaultSortDirection** son:

- MR: "Más reciente"
- NF: "Más recientes primero"
- OF: "Más antiguos primero"

Esto también se puede hacer sin código. En la página de personalización del widget, consulta la sección "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Ten en cuenta que los comentarios en cada página para cada dirección de ordenación se precomputan, por lo que todas las direcciones de ordenación tienen el mismo rendimiento.