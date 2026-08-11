FastComments SSO (<a href="#sso">Details hier</a>) bietet Ihren Benutzern eine Möglichkeit, Kommentare zu hinterlassen, ohne sich bei einer anderen Plattform anmelden zu müssen.

Allerdings sichert dies allein Ihre Kommentar‑Threads nicht, da Kommentar‑Daten standardmäßig öffentlich zugänglich sind – jeder, der die Seite sehen kann, kann auch die Kommentare sehen.

Durch Ändern einer Einstellung können wir das Abrufen von Kommentaren einschränken, sodass dies nur von einem Administrator oder einem gültigen SSO‑Benutzer erfolgt.

#### Einrichtung ohne Code

Wir können das Anzeigen und Interagieren mit unseren Kommentar‑Threads verhindern, wenn SSO eingerichtet ist, indem wir eine <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Anpassungsregel</a> erstellen.

Suchen Sie dabei nach SSO, und Sie werden diese Option finden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Option SSO erforderlich, um Kommentare anzuzeigen in einer Anpassungsregel aktiviert, mit Auswahl des Sicherheitslevels'; title='SSO erforderlich, um Kommentare anzuzeigen' app-screenshot-end]

Aktivieren Sie sie und speichern Sie die Anpassungsregel.

#### Nur eine bestimmte Domain oder Seite schützen

Um nur eine bestimmte Domain oder Seite zu schützen, konfigurieren wir einfach die Anpassungsregel entsprechend.

Oben in der Anpassungs‑UI finden wir zwei Eingabefelder, Domain und URL‑ID.

Um nur eine bestimmte Domain zu schützen, geben Sie die betreffende Domain in das Feld „Domain“ ein.

Um eine bestimmte Seite zu schützen, geben Sie die Seiten‑URL in das Feld „URL‑ID“ ein. Wenn Sie eine benutzerdefinierte Integration mit FastComments haben, können Sie hier stattdessen eine Art von ID anstelle einer URL eingeben.

#### Sicherheitsstufen

Wenn SSO verlangt wird, sollten Sie entscheiden, ob Sie einfaches SSO oder sicheres SSO benötigen. Bei einfachem SSO sind beide zulässig, bei sicherem SSO muss der Inhalt mit einer sicheren SSO‑Payload, die mit Ihrem API‑Schlüssel gehasht ist, abgerufen werden, um angezeigt zu werden.

Die Option für das Sicherheitslevel erscheint, wenn Sie „SSO erforderlich, um Kommentare anzuzeigen“ auswählen.

#### Schutz über das Lesen hinaus

Das Aktivieren dieser Option schützt die Seite oder Domain davor, kommentiert zu werden, es sei denn, der Benutzer ist über SSO angemeldet.

#### Stolperfallen

Benutzer, die Kommentare vor Ihrer SSO‑Integration erstellt haben, können diese nicht mehr sehen, es sei denn, sie melden sich über Ihre SSO‑Integration an.