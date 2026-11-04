Die kleinstmögliche Erweiterung wäre:

[inline-code-attrs-start title = 'Eine einfache Erweiterung'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Für dieses Beispiel speichern Sie dies als `my-extension.js` und machen es unter `https://example.com/my-extension.min.js` verfügbar.

Diese Erweiterung macht nichts, außer dass sie beim Laden das von der Core-Kommentarbibliothek erstellte Erweiterungsobjekt abruft.

Dieses `Extension`-Objekt ist ein Singleton und wird mit keiner anderen Erweiterung geteilt.

Als Nächstes müssen wir, um unsere Erweiterung zu laden, das Kommentar-Widget darüber informieren. Zum Beispiel:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Verwendung einer benutzerdefinierten Erweiterung'; code-example-end]

Funktionale Beispiele finden Sie im nächsten Abschnitt.