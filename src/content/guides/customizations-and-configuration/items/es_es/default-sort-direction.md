[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Por defecto, FastComments ordenará los comentarios por la dirección de ordenación "Most Relevant".

El ordenamiento Most Relevant tiene en cuenta la hora en que se dejó el comentario y el número de votos para ordenar.

El usuario puede entonces cambiar la dirección de ordenación a "Oldest" o "Newest First" en la interfaz del widget de comentarios.

Sin embargo, podemos cambiar el valor predeterminado a cualquiera de los tres. Por ejemplo, si deseas mostrar primero los comentarios más antiguos:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Cambiar la Ordenación Predeterminada a Más Antiguos Primero'; code-example-end]

Establecemos el valor de **defaultSortDirection** a "OF" para fijar la dirección a "OF".

Para la dirección de ordenación newest-first, haríamos lo siguiente:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Cambiar la Ordenación Predeterminada a Más Recientes Primero'; code-example-end]

Los valores válidos para **defaultSortDirection** son:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Esto también se puede hacer sin código. En la página de personalización del widget, consulta la sección "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Selector de Default Sort Direction que ofrece Most Relevant, Newest First y Oldest First'; title='Cambiar la Dirección de Ordenación Predeterminada' app-screenshot-end]

Tenga en cuenta que los comentarios en cada página para cada dirección de ordenación se pre‑calculan, por lo que todas las direcciones de ordenación tienen el mismo rendimiento.