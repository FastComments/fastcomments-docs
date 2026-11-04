FastComments SSO (<a href="#sso">Details hier</a>) bietet Ihren Benutzern eine Möglichkeit, Kommentare zu schreiben, ohne sich bei einer anderen Plattform anmelden zu müssen.

Allerdings sichert dies allein Ihre Kommentar‑Threads nicht, da Kommentar­daten standardmäßig öffentlich zugänglich sind – jeder, der die Seite sehen kann, kann die Kommentare sehen.

Durch Ändern einer Einstellung können wir das Abrufen von Kommentaren einschränken, sodass dies nur durch einen Administrator oder einen gültigen SSO‑Benutzer erfolgt.

#### No‑Code‑Setup

Wir können das Anzeigen und Interagieren mit unseren Kommentar‑Threads verhindern, wenn SSO eingerichtet ist, indem wir eine <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Anpassungsregel</a> erstellen.

Suchen Sie dabei nach SSO, und Sie finden diese Option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Option \'SSO erforderlich, um Kommentare anzuzeigen\' in einer Anpassungsregel aktiviert, mit der Auswahl des Sicherheitslevels'; title='SSO erforderlich, um Kommentare anzuzeigen' app-screenshot-end]

Aktivieren Sie sie und speichern Sie die Anpassungsregel.

#### Nur einen bestimmten Domain oder eine bestimmte Seite schützen

Um nur einen bestimmten Domain oder eine bestimmte Seite zu schützen, konfigurieren wir einfach die Anpassungsregel entsprechend.

Oben in der Anpassungs‑UI finden wir zwei Eingabefelder: Domain und URL‑ID.

Um nur einen bestimmten Domain zu schützen, geben Sie den betreffenden Domain in das Feld „domain“ ein.

Um eine bestimmte Seite zu schützen, geben Sie die Seiten‑URL in das Feld „URL ID“ ein. Wenn Sie eine benutzerdefinierte Integration mit FastComments haben, können Sie hier stattdessen eine Art von ID anstelle einer URL eingeben.

#### Sicherheitsstufen

Wenn SSO verlangt wird, müssen Sie entscheiden, ob Sie Simple SSO oder Secure SSO benötigen. Bei Simple SSO sind beide zulässig, bei Secure SSO muss der Inhalt mit einer Secure‑SSO‑Payload, die mit Ihrem API‑Schlüssel gehasht ist, abgerufen werden, um angezeigt zu werden.

Die Sicherheitsstufen‑Option erscheint, wenn Sie „SSO erforderlich, um Kommentare anzuzeigen“ auswählen, und wirkt nur, solange diese Option aktiviert ist.

Die <a href="#sso-simple">Kommentare automatisch verifizieren über Simple SSO und Simple SSO‑Admins zulassen</a>-Einstellungen sind separat und gelten unabhängig davon, ob diese Option aktiviert ist.

#### Schutz über das Lesen hinaus

Durch Aktivieren dieser Option wird die Seite oder der Domain davor geschützt, kommentiert zu werden, es sei denn, der Benutzer ist über SSO angemeldet.

#### Gotchas

Alle Benutzer, die Kommentare vor Ihrer SSO‑Integration erstellt haben, können diese nicht mehr sehen, es sei denn, sie melden sich über Ihre SSO‑Integration an.