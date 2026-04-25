Eine Teilmenge von Moderationsaktionen kann direkt im Kommentar-Thread selbst durchgeführt werden, ohne zur Comment Moderation-Seite gehen zu müssen.

Wenn Sie angemeldet sind, klicken Sie auf die Bearbeiten-Schaltfläche oben rechts an einem Kommentar. Als Moderator sollten Sie die folgenden Optionen haben:

- **Diesen Kommentar anpinnen**
- **Diesen Kommentar löschen**
- **Diesen Kommentar löschen** + **Den Nutzer sperren** (Permanent oder Shadow, mehr Details weiter unten)
- **Diesen Kommentar bearbeiten**
- **Diesen Kommentar sperren** oder **entsperren** (mehr Details weiter unten)
- **Diesen Kommentar als Genehmigt markieren** (anzeigen) oder **als Nicht genehmigt markieren** (ausblenden)
- **Diesen Kommentar als Spam markieren** oder **als Nicht-Spam markieren**

### Sperren eines Kommentars

Das Sperren eines einzelnen Kommentars verhindert neue Antworten darauf und verhindert außerdem, dass der Kommentar selbst bearbeitet oder gelöscht wird, bis er wieder entsperrt ist. Dies gilt für alle, einschließlich Administratoren und Moderatoren. Wenn Sie einen gesperrten Kommentar bearbeiten oder entfernen müssen, entsperren Sie ihn zuerst, nehmen Sie die Änderung vor und sperren Sie ihn bei Bedarf wieder.

Ein Schloss-Symbol erscheint oben rechts an einem gesperrten Kommentar, sodass Leser auf einen Blick sehen können, dass der Thread geschlossen ist. Die Menüeinträge Edit und Delete sind für gesperrte Kommentare sowohl im Kommentar-Widget als auch in der öffentlichen API (`PATCH` und `DELETE` return `code: 'locked'`, wenn sie gegen einen gesperrten Kommentar aufgerufen werden) versteckt.

Zwei beabsichtigte Ausnahmen umgehen die Sperre, da sie sonst verwaiste Daten hinterlassen würden: wenn ein Nutzer sein gesamtes Konto löscht (seine Kommentare werden unabhängig vom Sperrstatus bereinigt), und wenn ein Moderator einen Nutzer mit der Option "delete all comments from this user" sperrt (der Sweep entfernt die Sperren).

### Schließen von Kommentar-Threads

Moderatoren und Administratoren können Kommentar-Threads sperren bzw. schließen, indem sie `Close Thread` im Drei-Punkte-Menü oben im Kommentarbereich auswählen, wenn sie angemeldet sind. Sie können später jederzeit `Re-Open Thread` auswählen, um das Kommentieren wieder zu ermöglichen.

Das Schließen eines Kommentar-Threads verhindert neue Kommentare, erlaubt jedoch weiterhin Abstimmungen und den Nutzern, ihre Kommentare gegebenenfalls zu löschen.

Das Schließen und Wiederöffnen von Kommentar-Threads wirkt sich sofort auf alle Benutzer aus, die den Thread gerade ansehen.

Sie können einen Thread auch als schreibgeschützt markieren, wodurch ebenfalls Abstimm- und Löschoptionen entfernt werden, indem Sie eine Anpassungsregel speziell für diese Seite erstellen.

### Live-Aktualisierung

Alle diese Aktionen aktualisieren die Kommentar-Threads anderer Benutzer sofort, ohne dass diese die Seite neu laden müssen. Moderator-Aktionen wie das Ausblenden eines Kommentars oder das Markieren als Spam entfernen den Kommentar jedoch nicht vom Bildschirm **des Moderators**, damit er die Aktion bei Bedarf schnell rückgängig machen kann. Um anzuzeigen, dass ein Kommentar ausgeblendet ist, wird er im Vergleich zu den anderen Kommentaren hervorgehoben (die Hervorhebungsfarbe hängt vom Entferngrund ab).

For example, given users `A (commenter)`, `B (Moderator 1)`, and `C (Moderator 2)`.

...and the following scenario:

1. `User B (Moderator 1)` blendet einen Kommentar aus.
2. Für `User A (commenter)` wird dieser Kommentar sofort ausgeblendet.
3. Für `User C (Moderator 2)` wird dieser Kommentar sofort ausgeblendet.
4. Für den Benutzer, der die Änderung vorgenommen hat, `User B (Moderator 1)`, bleibt der Kommentar auf seinem Bildschirm, wird jedoch als entfernt hervorgehoben. Er hat die Möglichkeit, seine Aktion rückgängig zu machen; in diesem Fall sehen die anderen Benutzer das Update wieder live.