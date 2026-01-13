L'estensione più piccola possibile sarebbe:

[inline-code-attrs-start title = 'Una semplice estensione'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Per questo esempio, salva questo come `my-extension.js`, e rendilo disponibile a `https://example.com/my-extension.min.js`.

Questa estensione non fa nulla; al caricamento recupera l'oggetto extension creato dalla libreria principale dei commenti.

Questo oggetto `Extension` è un singleton e non viene condiviso con altre estensioni.

Successivamente, per caricare la nostra estensione, dobbiamo comunicarlo al widget dei commenti. Ad esempio:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Per esempi funzionali, vedi la sezione successiva.