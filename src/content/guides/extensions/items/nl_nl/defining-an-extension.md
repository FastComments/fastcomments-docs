De kleinste mogelijke extensie zou zijn:

[inline-code-attrs-start title = 'Een eenvoudige extensie'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Sla dit voor dit voorbeeld op als `my-extension.js` en maak het beschikbaar via `https://example.com/my-extension.min.js`.

Deze extensie doet niets; bij het laden haalt hij het extensieobject op dat door de kern van de commentaarbibliotheek is aangemaakt.

Dit `Extension`-object is een singleton en wordt niet gedeeld met andere extensies.

Vervolgens, om onze extensie te laden, moeten we de commentaarwidget hierover informeren. Bijvoorbeeld:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Voor functionele voorbeelden, zie de volgende sectie.