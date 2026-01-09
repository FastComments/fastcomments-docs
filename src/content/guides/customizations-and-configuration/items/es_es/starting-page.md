[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Al recuperar y renderizar los comentarios, el widget de comentarios necesita saber en qué página empezar. Por defecto, comienza en la primera página, renderizando únicamente esa página.

Si se desea, la página exacta que debe renderizarse puede pasarse al widget de comentarios como la configuración *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Tenga en cuenta que los números de página comienzan en cero, por lo que el ejemplo anterior renderiza la segunda página.