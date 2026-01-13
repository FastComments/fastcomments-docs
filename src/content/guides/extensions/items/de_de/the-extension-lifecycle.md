Das Skript für jede Erweiterung wird abgerufen und aufgerufen, bevor das Kommentar-Widget beginnt, das erste Set von Kommentaren abzurufen und die Benutzeroberfläche zu rendern.

Beim ersten Laden werden die folgenden Daten dem Erweiterungsobjekt angehängt:

- `config` - Eine Referenz auf das `config`-Objekt.
- `translations` - Eine Referenz auf das `translations`-Objekt.
- `commentsById` - Eine Referenz auf alle Kommentare nach ID.
- `root` - Eine Referenz auf den Root-DOM-Knoten.

Erweiterungen sollten die gewünschten Funktionen überschreiben, die das Kommentar-Widget zu den entsprechenden Zeitpunkten aufrufen wird.