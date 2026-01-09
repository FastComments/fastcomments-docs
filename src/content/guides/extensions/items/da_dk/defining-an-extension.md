---
Den mindste mulige udvidelse er:

[inline-code-attrs-start title = 'En simpel udvidelse'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

For eksemplets skyld, gem dette som `my-extension.js`, og gør det tilgængeligt på `https://example.com/my-extension.min.js`.

Denne udvidelse gør ikke noget; ved indlæsning henter den udvidelsesobjektet, som er oprettet af kernekommentarbiblioteket.

This `Extension` object is a singleton and is not shared with any other extensions.

Dernæst, for at indlæse vores udvidelse, skal vi fortælle kommentar-widgeten om den. For eksempel:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

For funktionelle eksempler, se næste afsnit.

---