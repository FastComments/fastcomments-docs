All settings live under `Administration > Konfiguration > Inhalt > FastComments` (`/admin/config/content/fastcomments`).

## Erforderlich

- **Tenant ID** – Ihre FastComments Tenant ID. Finden Sie diese unter [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** – Erforderlich für Secure SSO, Webhook‑Überprüfung und Seiten‑Synchronisation. Gefunden unter [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Speichern des API Secret

Das Modul liest das API Secret aus der ersten dieser Quellen, die einen Wert enthält:

1. **Key‑Modul** – Installieren Sie [Key](https://www.drupal.org/project/key), erstellen Sie einen *Authentication*‑Schlüssel und wählen Sie ihn unter **API Secret key** im Einstellungsformular aus. Der Schlüssel kann durch eine Umgebungsvariable oder eine Datei außerhalb des Web‑Root unterstützt werden.
2. **settings.php** – Fügen Sie `$config['fastcomments.settings']['api_secret'] = 'your-secret';` zu `settings.php` oder `settings.local.php` hinzu. Das Geheimnis wird nie in exportierte Konfiguration aufgenommen.
3. **Das Einstellungsformular** – Geben Sie es in das Feld **API Secret** ein. Dies speichert den Wert in `fastcomments.settings`, sodass er in Konfigurationsexporte gelangt. Der Drupal‑Statusbericht warnt, wenn dies die aktive Quelle ist.

Das Formular zeigt, welche Quelle aktiv ist. Das Feld leer zu lassen bewahrt das gespeicherte Geheimnis. Aktivieren Sie **Remove the API Secret stored in configuration**, um es zu löschen, nachdem das Geheimnis in das Key‑Modul oder `settings.php` verschoben wurde.

## Kommentarstil

Wählen Sie das Widget, das zu der Art und Weise passt, wie Besucher auf Ihrer Seite kommunizieren sollen.

- **Live Comments** – Echtzeit‑Threaded‑Kommentare.
- **Streaming Chat** – Live‑Chat‑Oberfläche, gut für Veranstaltungen und Livestreams.
- **Collab Chat** – Textauswahl‑Annotation im Hauptinhaltsbereich. Besucher markieren Text und starten eine Diskussion im Kontext.
- **Collab Chat + Comments** – Sowohl Collab‑Chat als auch Standardkommentare auf derselben Seite.

## SSO‑Modus

- **None** – Kein SSO. Benutzer kommentieren als Gäste oder erstellen ein FastComments‑Konto.
- **Simple** – Überträgt Drupal‑Benutzerinformationen (Name, E‑Mail, Avatar) an FastComments ohne serverseitige Verifizierung.
- **Secure** – Verwendet HMAC‑SHA256, um Drupal‑Benutzer mit FastComments zu verifizieren. Empfohlen, wenn ein API Secret konfiguriert ist.

Siehe den Abschnitt `Single Sign-On (SSO)` für Details.

## Weitere Einstellungen

- **CDN URL** – Standardwert ist `https://cdn.fastcomments.com`.
- **Site URL** – Standardwert ist `https://fastcomments.com`.
- **Email notifications** – Sendet eine E‑Mail an den Inhaltsautor, wenn ein neuer Kommentar zu seinem Inhalt gepostet wird.

Für EU‑Datenresidenz siehe den Abschnitt `EU Data Residency`.

---