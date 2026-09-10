[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Standardmäßig geht ein Entwurf, den ein Benutzer eingibt, stillschweigend verloren, wenn er die Seite aktualisiert, den Tab schließt oder die Seite verlässt, bevor er ihn absendet.

Wenn **warnOnUnsavedComment** auf true gesetzt wird, fragt der Browser den Benutzer, ob er die Seite verlassen möchte, solange ein Kommentarfeld oder eine laufende Bearbeitung noch Text enthält. Sobald der Kommentar abgesendet wurde, wird der Text gelöscht, sodass keine Eingabeaufforderung mehr angezeigt wird.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Warnung bei nicht gespeichertem Kommentar'; code-example-end]

Die Eingabeaufforderung verwendet den nativen Dialog des Browsers. Moderne Browser zeigen ihre eigene Formulierung an und ignorieren benutzerdefinierten Text, sodass die Meldung nicht angepasst werden kann.

Diese Option lädt bei Bedarf eine kleine Erweiterung, sodass sie für Websites, die sie nicht aktivieren, nichts zum Widget hinzufügt.