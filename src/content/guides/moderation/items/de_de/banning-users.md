Es gibt zwei Möglichkeiten, Benutzer davon abzuhalten, auf Ihrer Seite mit FastComments Kommentare zu schreiben.

Die erste Möglichkeit besteht darin, wenn Sie bereits deren E‑Mail‑Adresse kennen, können Sie sie auf der <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">gesperrte Benutzer</a>-Seite eingeben.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Liste der gesperrten Benutzer unter Moderierte Kommentare, mit den gesperrten E‑Mail‑Adressen und einer Schaltfläche zum Hinzufügen einer neuen Sperre'; title='Die Seite Gesperrte Benutzer' app-screenshot-end]

Diese Seite kann über Moderierte Kommentare -> Gesperrte Benutzer aufgerufen werden.

Wenn wir einen Benutzer sperren, können wir einen Typ auswählen, entweder Permanent oder Permanenter Schattenbann:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Neues Sperrformular mit einem E‑Mail‑Feld und einer Auswahl des Sperrtyps Permanent oder Permanenter Schattenbann'; title='Einen Benutzer sperren' app-screenshot-end]

Die zweite Möglichkeit, einen Benutzer zu sperren, besteht darin, den Sperrknopf zu klicken, der bei jedem Kommentar auf der Seite Kommentar‑Moderation platziert ist.

Wenn wir den Sperrknopf klicken, werden Ihnen einige Optionen angezeigt, bei denen wir den Sperrtyp und die Dauer festlegen können.

### E‑Mail‑Aliase

Beim Sperren eines Benutzers per E‑Mail ignoriert FastComments automatisch `+`‑Aliase. Zum Beispiel führt das Sperren von `user+alias@gmail.com` auch dazu, dass `user@gmail.com` und jede andere `+`‑Variante dieser Adresse gesperrt wird, wie `user+other@gmail.com`.

### Schattenbann

Ein Schattenbann ist eine Art von Sperre, die den Anschein erweckt, dass der Kommentar oder die Stimme des Benutzers erfolgreich gespeichert wurde, obwohl dies nicht der Fall ist. Dies kann in bestimmten Situationen wünschenswert sein.

### Sperren per IP‑Adresse

Sofern ein Mandant nicht opt‑out wählt, unterstützt FastComments das Sperren per IP, indem eine gehashte Version der IP‑Adresse des Kommentators gespeichert wird.