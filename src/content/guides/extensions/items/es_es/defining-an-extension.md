La extensión más pequeña posible sería:

[inline-code-attrs-start title = 'Una extensión simple'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Para este ejemplo, guárdalo como `my-extension.js`, y ponlo disponible en `https://example.com/my-extension.min.js`.

Esta extensión no hace nada; excepto que al cargarse recupera el objeto de extensión creado por la biblioteca principal de comentarios.

Este objeto `Extension` es una instancia única y no se comparte con ninguna otra extensión.

A continuación, para cargar nuestra extensión, tenemos que informar al widget de comentarios sobre ella. Por ejemplo:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Para ejemplos funcionales, vea la siguiente sección.

---