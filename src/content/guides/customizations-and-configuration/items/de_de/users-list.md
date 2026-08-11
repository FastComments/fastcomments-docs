[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments keine Benutzerliste auf der Seite an.

Sie können eine Liste von Personen anzeigen, die die Seite gerade ansehen, zusammen mit dem Kommentar-Widget. Die Liste wird live aktualisiert, wenn Benutzer hinzukommen oder gehen, und zeigt deren Namen, Avatar und einen Online‑Indikator.

Es gibt drei Layout‑Optionen:

- `1` - Oben: eine horizontale Reihe überlappender Avatare, die über den Kommentaren angezeigt wird.
- `2` - Links: eine Seitenleiste mit Namen und Online‑Punkten, die links vom Widget angezeigt wird.
- `3` - Rechts: dieselbe Seitenleiste, die rechts vom Widget angezeigt wird.

Setzen Sie das **usersListLocation**‑Flag, um die Funktion zu aktivieren:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Benutzerliste rechts anzeigen'; code-example-end]

Standardmäßig zeigt die Liste nur derzeit online befindliche Benutzer an. Um auch Personen einzubeziehen, die in der Vergangenheit auf der Seite kommentiert haben (aber die Seite gerade nicht ansehen), setzen Sie **usersListIncludeOffline** auf true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Frühere Kommentatoren einbeziehen'; code-example-end]

Frühere Kommentatoren werden ohne den grünen Online‑Punkt angezeigt, sodass klar ist, wer gerade anwesend ist.

Benutzer mit privaten Profilen werden mit einem generischen Avatar und einem "Privates Profil"-Label angezeigt, sodass die Zählung genau bleibt, ohne Identitäten preiszugeben.

Dies kann auch ohne Code konfiguriert werden. Auf der Widget‑Anpassungsseite finden Sie die Option "Users List Location". Wenn der Ort auf etwas anderes als Off eingestellt ist, erscheint darunter ein Kontrollkästchen "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Users List Location auf Rechts eingestellt, mit dem darunter angezeigten Kontrollkästchen \"Include past commenters\"'; title='Einstellungen der Benutzerliste'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Bei mehr als 500 Live‑Benutzern ist die Liste bis zu 30 Sekunden veraltet.