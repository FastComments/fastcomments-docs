[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments keine Liste von Benutzern auf der Seite an.

Sie können neben dem Kommentar-Widget eine Liste der Personen anzeigen, die die Seite gerade betrachten. Die Liste aktualisiert sich live, wenn Benutzer hinzukommen oder die Seite verlassen, und zeigt deren Namen, Avatar und einen Online-Indikator an.

Es gibt drei Layout-Optionen:

- `1` - Top: eine horizontale Reihe überlappender Avatare, die über den Kommentaren angezeigt werden.
- `2` - Left: eine Seitenleiste mit Namen und Online-Punkten, die links vom Widget angezeigt wird.
- `3` - Right: dieselbe Seitenleiste, die rechts vom Widget angezeigt wird.

Setzen Sie die **usersListLocation**-Flag, um die Funktion zu aktivieren:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Standardmäßig zeigt die Liste nur aktuell online befindliche Benutzer an. Um außerdem Personen einzubeziehen, die in der Vergangenheit auf der Seite kommentiert haben (aber sich derzeit nicht darauf befinden), setzen Sie **usersListIncludeOffline** auf true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Frühere Kommentatoren werden ohne den grünen Online-Punkt dargestellt, damit klar ersichtlich ist, wer gerade anwesend ist.

Benutzer mit privaten Profilen werden mit einem generischen Avatar und dem Label "Private Profile" angezeigt, damit die Anzahl korrekt bleibt, ohne Identitäten preiszugeben.

Dies kann auch ohne Code konfiguriert werden. Auf der Seite zur Widget-Anpassung sehen Sie die Option "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Wenn die Position auf etwas anderes als Off eingestellt ist, wird darunter das Kontrollkästchen "Include past commenters" angezeigt:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]