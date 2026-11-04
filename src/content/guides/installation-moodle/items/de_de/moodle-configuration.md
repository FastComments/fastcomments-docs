Die Plugin-Einstellungsseite befindet sich unter **Site Administration > Plugins > Local plugins > FastComments**. Die verfügbaren Optionen sind:

#### Tenant ID

Ihre FastComments Tenant ID. Sie finden diese im <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments-Dashboard</a> unter Ihren Kontoeinstellungen.

#### API Secret

Ihr API-Secret-Schlüssel, erforderlich für den Secure-SSO-Modus. Sie finden ihn unter <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Wählen Sie, wie Benutzer authentifiziert werden. Details zu jeder Option finden Sie im Abschnitt [SSO Modes](#moodle-sso-modes).

- **Secure** (empfohlen) - serverseitig HMAC-SHA256-signierte Authentifizierung
- **Simple** - clientseitige Benutzerdaten ohne Signatur
- **None** - anonymes Kommentieren, keine Moodle-Login-Integration

#### Page Contexts

Steuert, wo Kommentare erscheinen:

- **Course pages** - Kommentare auf den Hauptseiten von Kursen
- **Module/activity pages** - Kommentare zu einzelnen Aktivitäten und Ressourcen
- **Both** - Kommentare auf allen Seitentypen

#### Commenting Style

Wählen Sie das Kommentierungserlebnis. Screenshots der einzelnen Modi finden Sie unter [Commenting Styles](#moodle-commenting-styles).

- **Comments** - Standard-Threaded-Comment-Widget unter dem Seiteninhalt
- **Collab Chat** - Inline-Diskussionen zu Textauswahlen mit Anwesenheitsindikatoren
- **Both** - Kommentare und Collab Chat gleichzeitig aktiv

#### CDN URL

Die FastComments-CDN-URL. Standardmäßig `https://cdn.fastcomments.com`. Ändern Sie dies zur EU-CDN-URL, wenn Ihre Daten in der EU-Region gehostet werden.