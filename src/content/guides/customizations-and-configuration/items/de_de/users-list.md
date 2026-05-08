[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments keine Benutzerliste auf der Seite an.

Sie können neben dem Kommentar-Widget eine Liste der Personen anzeigen, die die Seite gerade ansehen. Die Liste wird in Echtzeit aktualisiert, wenn Nutzer hinzukommen oder die Seite verlassen, und zeigt deren Namen, Avatar und einen Online-Indikator.

Es gibt drei Layout-Optionen:

- `1` - Oben: eine horizontale Reihe überlappender Avatare, die über den Kommentaren angezeigt wird.
- `2` - Links: eine Seitenleiste mit Namen und Online-Indikatoren, die links vom Widget angezeigt wird.
- `3` - Rechts: dieselbe Seitenleiste, die rechts vom Widget angezeigt wird.

Setzen Sie das **usersListLocation**-Flag, um die Funktion zu aktivieren:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Standardmäßig zeigt die Liste nur aktuell online befindliche Nutzer. Um auch Personen einzubeziehen, die in der Vergangenheit auf der Seite kommentiert haben (aber gerade nicht darauf zugreifen), setzen Sie **usersListIncludeOffline** auf true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Frühere Kommentatoren werden ohne den grünen Online-Punkt dargestellt, sodass klar erkennbar ist, wer gerade anwesend ist.

Nutzer mit privaten Profilen werden mit einem generischen Avatar und dem Label "Privates Profil" angezeigt, sodass die Anzahl korrekt bleibt, ohne Identitäten preiszugeben.

Dies kann auch ohne Code konfiguriert werden. Auf der Seite zur Anpassung des Widgets finden Sie die Option "Position der Benutzerliste". Wenn die Position auf etwas anderes als Aus gesetzt ist, erscheint darunter ein Kontrollkästchen "Frühere Kommentatoren einschließen".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---