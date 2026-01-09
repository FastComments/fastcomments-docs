[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Standardmäßig erlaubt FastComments unbegrenzte Verschachtelung von Antworten und erzeugt so eine Thread-Struktur, in der Benutzer Antworten auf Antworten unbegrenzt schreiben können.

Die Option maxReplyDepth ermöglicht es Ihnen, die Tiefe von Antwort-Threads zu begrenzen. Wenn die maximale Tiefe erreicht ist, sehen Benutzer bei Kommentaren auf dieser Ebene keine Antwort-Schaltfläche mehr.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Bei maxReplyDepth = 2:
- Benutzer können auf oberster Ebene kommentieren (Tiefe 0)
- Benutzer können auf Kommentare der obersten Ebene antworten (Tiefe 1)
- Benutzer können auf diese Antworten antworten (Tiefe 2)
- Weitere Antworten über Tiefe 2 hinaus sind nicht erlaubt

Das Setzen auf 1 würde nur Antworten auf Kommentare der obersten Ebene erlauben und eine flachere Diskussionsstruktur schaffen.

Das Setzen von maxReplyDepth auf 0 würde alle Antworten deaktivieren und nur Kommentare auf oberster Ebene erlauben. Wird es nicht angegeben, können Antworten unbegrenzt verschachtelt werden.