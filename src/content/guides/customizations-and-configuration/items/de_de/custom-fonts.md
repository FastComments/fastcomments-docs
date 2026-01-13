[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments ist so konzipiert, dass es angepasst werden kann, und die Schriftart, die unsere Widgets verwenden, bildet da keine Ausnahme.

Standardmäßig verwendet FastComments die `system font stack`, damit es auf einer Vielzahl von Geräten bestmöglich aussieht.

Um eigene Schriftarten zu definieren, siehe die [Dokumentation für benutzerdefiniertes CSS](/guide-customizations-and-configuration.html#custom-css).

Dort finden Sie eine Möglichkeit, benutzerdefiniertes CSS zu definieren, mit dem Sie Ihre gewünschten Schriftarten festlegen können.

#### So definieren Sie die Schriftart

Um die Schriftart zu überschreiben, empfehlen wir, Ihr CSS mit den Selektoren `.fast-comments, textarea` zu definieren. Zum Beispiel:

[inline-code-attrs-start title = 'Beispiel für externe benutzerdefinierte Schriftart'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---