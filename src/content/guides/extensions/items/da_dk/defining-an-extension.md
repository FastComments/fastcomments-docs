Den mindst mulige udvidelse ville være:

[inline-code-attrs-start title = 'En simpel udvidelse'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

I dette eksempel kan du gemme dette som `my-extension.js` og gøre det tilgængeligt på `https://example.com/my-extension.min.js`.

Denne udvidelse gør ikke noget, bortset fra at den ved indlæsning henter det udvidelsesobjekt, der er oprettet af kerne-kommentarbiblioteket.

Dette `Extension`-objekt er en singleton og deles ikke med andre udvidelser.

Dernæst, for at indlæse vores udvidelse, skal vi fortælle kommentar-widgetten om den. For eksempel:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Brug af en brugerdefineret udvidelse'; code-example-end]

For funktionelle eksempler, se næste afsnit.