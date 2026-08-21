Es gibt zwei Möglichkeiten, Benutzer davon abzuhalten, auf Ihrer Seite mit FastComments Kommentare zu schreiben.

Die erste ist, wenn Sie deren E‑Mail bereits kennen, können Sie sie auf der <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">Gesperrte‑Benutzer</a>-Seite eingeben.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Liste der gesperrten Benutzer unter Moderierte Kommentare, mit den gesperrten E-Mail-Adressen und einem Button zum Hinzufügen einer neuen Sperre'; title='Die Seite Gesperrte Benutzer' app-screenshot-end]

Diese Seite kann über Moderierte Kommentare -> Gesperrte Benutzer aufgerufen werden.

Wenn wir einen Benutzer sperren, können wir einen Typ auswählen, entweder Permanent oder Permanenter Schattenbann:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Neues Sperrformular mit einem E-Mail-Feld und einer Auswahl des Sperrtyps Permanent oder Permanenter Schattenbann'; title='Einen Benutzer sperren' app-screenshot-end]

Die zweite Möglichkeit, einen Benutzer zu sperren, besteht darin, den Sperr‑Button zu klicken, der bei jedem Kommentar auf der Seite Kommentar‑Moderation platziert ist.

Wenn wir den Sperr‑Button klicken, werden Ihnen einige Optionen angezeigt, bei denen wir den Sperrtyp und die Dauer festlegen können.

### E‑Mail‑Aliase

Beim Sperren eines Benutzers per E‑Mail ignoriert FastComments automatisch `+`‑Aliase. Zum Beispiel sperrt das Sperren von `user+alias@gmail.com` auch `user@gmail.com` und jede andere `+`‑Variante dieser Adresse, wie `user+other@gmail.com`.

### Schattenbanns

Ein Schattenbann ist eine Art von Sperre, die den Anschein erweckt, dass der Kommentar oder die Stimme des Benutzers erfolgreich gespeichert wurde, obwohl dies nicht der Fall war. Dies kann in bestimmten Situationen wünschenswert sein.

### Sperren per IP‑Adresse

Sofern ein Mandant nicht opt-out wählt, unterstützt FastComments das Sperren per IP, indem eine gehashte Version der IP‑Adresse des Kommentators gespeichert wird.

### Gesperrte Benutzer suchen

Wenn Ihre Liste mehr als eine oder zwei Seiten umfasst, können Sie sie mit der Suchzeile über der Tabelle eingrenzen.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Suchzeile auf der Seite Gesperrte Benutzer mit einem Dropdown \'Search By\', einem Dropdown \'Match\' und einem Eingabefeld \'Value\''; title='Gesperrte Benutzer suchen' app-screenshot-end]

Es gibt drei Steuerungen:

- **Search By** wählt das Feld, in dem gesucht werden soll: Any Field, Email, Name, Banned By oder Banned For Saying. Die letzten vier entsprechen den gleichnamigen Spalten in der Tabelle.
- **Match** legt fest, wie verglichen wird. **Contains** findet Ihren Wert irgendwo im Feld, und **Equals** stimmt mit dem gesamten Feld überein.
- **Value** ist der zu suchende Text.

Jedes Feld wird ohne Berücksichtigung der Groß‑ und Kleinschreibung verglichen, sodass die Suche nach `SPAMMER@EXAMPLE.COM` eine Sperre findet, die als `spammer@example.com` gespeichert ist.

Einige wichtige Hinweise:

- **Banned For Saying** durchsucht den Text des Kommentars, der den Benutzer gesperrt hat. So finden Sie alle gesperrt, die eine bestimmte Phrase enthalten.
- **Banned By** durchsucht den Namen des Moderators, der die Sperre ausgesprochen hat, was nützlich ist, um die Entscheidungen eines anderen Moderators zu überprüfen.
- Platzhalter‑Sperren werden mit ihrem `*` gespeichert, sodass eine **Contains**‑Suche nach `bademail.com` eine `*@bademail.com`‑Sperre findet.
- **Name** stimmt mit dem im Namens‑Spalte angezeigten Namen überein, sodass ein Benutzer gefunden wird, selbst wenn er seinen Namen seit der Sperrung geändert hat, und selbst wenn Sie die Sperre durch Eingabe einer E‑Mail‑Adresse erstellt haben und zum Zeitpunkt kein Name erfasst wurde. Der bei der Sperre gespeicherte Name stimmt ebenfalls, sodass die Suche nach dem alten oder dem aktuellen Namen funktioniert.
- **Any Field** durchsucht gleichzeitig die E‑Mail, den Namen, den sperrenden Moderator und den Text des gesperrten Kommentars.

Ihre Suche ist Teil der Seiten‑URL, sodass Sie eine gefilterte Liste mit anderen Moderatoren auf dieselbe Weise teilen können, wie Sie andere Moderations‑Links teilen. Das Durchblättern der Ergebnisse behält die Suche bei, ein neuer Suchvorgang führt Sie zurück zur ersten Seite, und **Clear** stellt die vollständige Liste wieder her.