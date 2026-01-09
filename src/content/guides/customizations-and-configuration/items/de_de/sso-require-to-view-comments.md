FastComments SSO (<a href="#sso">weitere Details</a>) bietet Ihren Nutzern eine Möglichkeit, Kommentare zu schreiben, ohne sich bei einer anderen Plattform anmelden zu müssen.

Allein dadurch sind Ihre Kommentarthreads jedoch nicht gesichert, da standardmäßig Kommentardaten öffentlich zugänglich sind - jeder, der die Seite sehen kann, kann auch die Kommentare sehen.

Durch Ändern einer Einstellung können wir verhindern, dass Kommentare abgerufen werden, es sei denn, der Abruf erfolgt durch einen Administrator oder einen gültigen SSO-Benutzer.

#### Einrichtung ohne Code

Wir können das Anzeigen und Interagieren mit unseren Kommentarthreads, wenn SSO eingerichtet ist, verhindern, indem wir eine <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Anpassungsregel</a> erstellen.

Suchen Sie dabei nach SSO, und Sie finden diese Option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Aktivieren Sie sie und speichern Sie die Anpassungsregel.

#### Nur eine bestimmte Domain oder Seite schützen

Um nur eine bestimmte Domain oder Seite zu schützen, konfigurieren wir einfach die Anpassungsregel entsprechend.

Oben in der Anpassungsoberfläche finden wir zwei Eingabefelder, Domain und URL ID.

Um nur eine bestimmte Domain zu schützen, geben Sie die betreffende Domain in das Feld "domain" ein.

Um eine bestimmte Seite zu schützen, geben Sie eine Seiten-URL in das Feld "URL ID" ein. Wenn Sie eine benutzerdefinierte Integration mit FastComments haben, können Sie hier stattdessen eine Art von ID eingeben.

#### Sicherheitsstufen

Wenn SSO erforderlich ist, müssen Sie entscheiden, ob Sie Simple SSO oder Secure SSO verlangen. Wenn Sie Simple SSO verlangen, sind beide zulässig; wenn Sie Secure SSO verlangen, muss der Inhalt mit einer Secure SSO-Payload abgerufen werden, die mit Ihrem API key gehasht wurde, damit er angezeigt werden kann.

Die Option für das Sicherheitslevel erscheint, wenn Sie "Require SSO To View Comments" auswählen.

#### Schutz über das Lesen hinaus

Wenn Sie diese Option aktivieren, wird die Seite bzw. Domain vor Kommentaren geschützt, es sei denn, der Benutzer ist über SSO angemeldet.

#### Fallstricke

Alle Benutzer, die Kommentare vor Ihrer SSO-Integration erstellt haben, können diese nicht sehen, sofern sie sich nicht über Ihre SSO-Integration anmelden.