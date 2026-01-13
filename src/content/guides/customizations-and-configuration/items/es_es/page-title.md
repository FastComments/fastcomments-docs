[related-parameter-start name = 'pageTitle'; type = 'string'; related-parameter-end]

El título de la página actual está asociado con el **urlId** especificado y se guarda para su uso en las herramientas de moderación.

Por defecto, esto se obtiene de **document.title**.

Si lo desea, puede especificar su propio título de página de la siguiente manera:

[code-example-start config = {pageTitle: "Article 42"}; linesToHighlight = [6]; title = 'Specifying The Page Title'; code-example-end]