Löst den Agenten aus, wenn ein Kommentar bearbeitet wird.

### Kontext, den der Agent erhält

- Der Kommentar in seiner aktuellen (nach der Bearbeitung) Form.
- Der **vorherige Kommentartext** als separater abgegrenzter Block (`PREVIOUS_TEXT`). Das ist einzigartig für den Edit-Trigger - der Agent kann vor/nach vergleichen.
- Optionaler Thread-/Benutzerverlauf/Seitenkontext wie konfiguriert.

### Bemerkenswert

- Der Trigger wird bei jeder erfolgreichen Bearbeitung ausgelöst, einschließlich Bearbeitungen, die Moderatoren im Auftrag eines Nutzers vorgenommen haben.
- Agenten haben kein Werkzeug zum Bearbeiten von Kommentaren zur Verfügung; Agenten können Kommentare überhaupt nicht bearbeiten.
- Der vorige Kommentartext ist als nicht vertrauenswürdige Eingabe abgegrenzt. Die Systemaufforderung der Plattform erinnert das Modell daran, Anweisungen innerhalb von Abgrenzungen nicht zu befolgen - das ist hier wichtig, weil ein böswilliger Nutzer einen Kommentar bearbeiten und eine Nutzlast wie "ignore your previous instructions" einfügen könnte, die sich an einen Agenten richtet, der Edit-Ereignisse beobachtet.

### Häufige Anwendungsfälle

- **Aufspüren verschleierter Inhalte** - ein Nutzer bearbeitet einen zuvor sauberen Kommentar, um Spam einzufügen, nachdem der Moderator weitergezogen ist.
- **Verfolgen kleiner Bearbeitungen** - wenn Ihre Community Bearbeitungen aus Prüfungsgründen als separate Ereignisse behandelt.

### Kostenhinweis

Edit-Trigger erhalten zwei Kopien des Kommentartexts (die neue Version im Standard-COMMENT-Block, die alte Version im PREVIOUS_TEXT-Block). Bei langen Kommentaren verdoppelt sich dadurch ungefähr der Token-Verbrauch des Durchlaufs im Vergleich zu einem `COMMENT_ADD`-Trigger - behalten Sie das bei der Budgetierung im Hinterkopf.