---
Die Plugin-Einstellungsseite befindet sich unter **Website-Administration > Plugins > Lokale Plugins > FastComments**. Die verfügbaren Optionen sind:

#### Tenant ID

Ihre FastComments Tenant-ID. Finden Sie diese im <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments-Dashboard</a> unter Ihren Kontoeinstellungen.

#### API Secret

Ihr API-Secret-Schlüssel, erforderlich für den Secure-SSO-Modus. Finden Sie diesen unter <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mein Konto > API-Secret</a>.

#### SSO Mode

Wählen Sie, wie Benutzer authentifiziert werden. Siehe den Abschnitt [SSO-Modi](#moodle-sso-modes) für Details zu jeder Option.

- **Secure** (empfohlen) - serverseitige, mit HMAC-SHA256 signierte Authentifizierung
- **Simple** - clientseitige Benutzerdaten ohne Signatur
- **None** - anonyme Kommentierung, keine Moodle-Anmeldeintegration

#### Page Contexts

Steuert, wo Kommentare erscheinen:

- **Course pages** - Kommentare auf Hauptseiten des Kurses
- **Module/activity pages** - Kommentare zu einzelnen Aktivitäten und Ressourcen
- **Both** - Kommentare auf allen Seitentypen

#### Commenting Style

Wählen Sie das Kommentiererlebnis. Siehe [Kommentier-Stile](#moodle-commenting-styles) für Screenshots jeder Darstellung.

- **Comments** - standardmäßiges threaded Kommentar-Widget unter dem Seiteninhalt
- **Collab Chat** - Inline-Diskussionen per Textauswahl mit Präsenzanzeigen
- **Both** - Kommentare und Collab Chat gleichzeitig aktiv

#### CDN URL

Die FastComments-CDN-URL. Standardmäßig `https://cdn.fastcomments.com`. Ändern Sie dies auf die EU-CDN-URL, wenn Ihre Daten in der EU gehostet werden.

---