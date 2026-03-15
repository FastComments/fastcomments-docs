Navigieren Sie zu **Administration > Konfiguration > Inhalt > FastComments** (`/admin/config/content/fastcomments`).

### Einstellungen

- **Tenant-ID** (erforderlich) - Ihre FastComments Tenant ID. Finden Sie dies unter [Einstellungen > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API-Geheimnis** - Erforderlich für Secure SSO, Webhook-Verifizierung und Page Sync. Zu finden unter [Einstellungen > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO-Modus** - Single Sign-On-Integration:
  - **None** - Kein SSO, Benutzer kommentieren als Gäste oder erstellen FastComments-Konten.
  - **Simple** - Übermittelt Drupal-Benutzerinformationen (Name, E-Mail, Avatar) an FastComments ohne serverseitige Verifizierung.
  - **Secure** - Verwendet HMAC-SHA256-Verifizierung, um Drupal-Benutzer sicher bei FastComments zu authentifizieren (empfohlen).
- **Kommentarstil** - Der anzuzeigende Widget-Typ:
  - **Live Comments** - Echtzeit-Thread-Kommentare.
  - **Streaming Chat** - Live-Chat-Oberfläche.
  - **Collab Chat** - Kollaborative Textauswahl-Anmerkung im Hauptinhaltsbereich.
  - **Collab Chat + Comments** - Sowohl Collab-Chat als auch Standardkommentare.
- **CDN URL** - FastComments CDN-URL (Standard: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments Site-URL (Standard: `https://fastcomments.com`).
- **E-Mail-Benachrichtigungen** - Sendet eine E-Mail an Inhaltsautoren, wenn ein neuer Kommentar zu ihrem Inhalt veröffentlicht wird.

### Hinzufügen von Kommentaren zu Inhaltstypen

Fügen Sie das Feld **FastComments** zu Ihren Inhaltstypen über **Struktur > Inhaltstypen > [type] > Felder verwalten** hinzu. Das Feld verfügt über einen Status-Umschalter und einen optionalen benutzerdefinierten Bezeichner pro Entität.

### EU-Datenresidenz

Für die EU-Datenresidenz aktualisieren Sie:
- **CDN URL** zu `https://cdn-eu.fastcomments.com`
- **Site URL** zu `https://eu.fastcomments.com`