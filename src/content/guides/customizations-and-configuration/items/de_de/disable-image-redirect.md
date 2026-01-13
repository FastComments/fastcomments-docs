---
[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Standardmäßig erlaubt FastComments Benutzern, Bilder hochzuladen. Wenn ein Benutzer auf dieses Bild klickt, öffnet FastComments standardmäßig
einen neuen Tab, um das Bild vollständig anzuzeigen. Das Setzen dieses Flags auf true deaktiviert dieses Verhalten:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Wenn Sie nicht vorhaben, den Bildklick selbst abzufangen (siehe [onImageClicked](#callbacks)), empfehlen wir, dies mit etwas Styling zu kombinieren,
um den Eindruck zu vermeiden, dass das Bild anklickbar ist.

---