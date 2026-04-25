Alle Einstellungen befinden sich unter `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Erforderlich

- **Tenant ID** - Ihre FastComments Tenant ID. Finden Sie diese unter [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Erforderlich für Secure SSO, Webhook-Verifizierung und Seiten-Synchronisation. Zu finden unter [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Kommentarstil

Wählen Sie das Widget, das dazu passt, wie Besucher auf Ihrer Seite kommunizieren sollen.

- **Live Comments** - Echtzeit-Thread-Kommentare.
- **Streaming Chat** - Live-Chat-Oberfläche, gut für Veranstaltungen und Live-Streams.
- **Collab Chat** - Anmerkungen durch Textauswahl im Hauptinhalt. Besucher markieren Text und starten eine Diskussion im Kontext.
- **Collab Chat + Comments** - Sowohl Collab Chat als auch normale Kommentare auf derselben Seite.

## SSO-Modus

- **None** - Kein SSO. Nutzer kommentieren als Gäste oder erstellen ein FastComments-Konto.
- **Simple** - Übermittelt Drupal-Benutzerinformationen (Name, E-Mail, Avatar) an FastComments ohne serverseitige Verifikation.
- **Secure** - Verwendet HMAC-SHA256, um Drupal-Benutzer mit FastComments zu verifizieren. Empfohlen, wenn ein API Secret konfiguriert ist.

Siehe den Abschnitt `Single Sign-On (SSO)` für Details.

## Weitere Einstellungen

- **CDN URL** - Standardmäßig `https://cdn.fastcomments.com`.
- **Site URL** - Standardmäßig `https://fastcomments.com`.
- **Email notifications** - Sendet eine E-Mail an den Inhaltsautor, wenn ein neuer Kommentar zu seinem Inhalt veröffentlicht wird.

Für Informationen zur Datenresidenz in der EU siehe den Abschnitt `EU Data Residency`.