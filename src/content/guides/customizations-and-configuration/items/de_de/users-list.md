[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments keine Liste der Benutzer auf der Seite an.

Sie können eine Liste von Personen anzeigen, die die Seite gerade betrachten, neben dem Kommentar-Widget. Die Liste wird in Echtzeit aktualisiert, wenn Nutzer hinzukommen oder die Seite verlassen, und zeigt deren Namen, Avatar und eine Online-Anzeige.

Es gibt drei Layout-Optionen:

- `1` - Oben: eine horizontale Reihe von überlappenden Avataren, die oberhalb der Kommentare angezeigt wird.
- `2` - Links: eine Seitenleiste mit Namen und Online-Punkten, die links vom Widget angezeigt wird.
- `3` - Rechts: dieselbe Seitenleiste, die rechts vom Widget angezeigt wird.

Aktivieren Sie die Funktion, indem Sie das Flag **usersListLocation** setzen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Benutzerliste rechts anzeigen'; code-example-end]

Standardmäßig zeigt die Liste nur derzeit online befindliche Nutzer. Um auch Personen einzuschließen, die in der Vergangenheit auf der Seite kommentiert haben (aber sich gerade nicht auf der Seite befinden), setzen Sie **usersListIncludeOffline** auf true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Vergangene Kommentatoren einbeziehen'; code-example-end]

Frühere Kommentatoren werden ohne den grünen Online-Punkt dargestellt, sodass klar ist, wer gerade anwesend ist.

Nutzer mit privaten Profilen werden mit einem generischen Avatar und der Kennzeichnung "Privates Profil" angezeigt, sodass die Anzahl korrekt bleibt, ohne Identitäten preiszugeben.

Dies kann auch ohne Code konfiguriert werden. Auf der Seite zur Anpassung des Widgets finden Sie die Option "Position der Benutzerliste". Wenn die Position auf etwas anderes als "Aus" gesetzt ist, erscheint darunter ein Kontrollkästchen "Frühere Kommentatoren einbeziehen".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Einstellungen für die Benutzerliste'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Bei mehr als 500 gleichzeitig aktiven Nutzern kann die Liste bis zu 30 Sekunden veraltet sein.