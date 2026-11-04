FastComments unterstützt drei Authentifizierungsmodi:

1. **Anonym** -- kein SSO-Token; Benutzer erhalten sitzungsbasierte Identitäten
2. **Einfaches SSO** -- clientseitiges Token für Demos und Tests (nicht sicher)
3. **Sicheres SSO** -- serverseitig signiertes Token für die Produktion

### Einfaches SSO

Nützlich für Demos und lokales Testen. Mit Simple SSO kann sich jeder als beliebiger Benutzer ausgeben, verwenden Sie es daher nicht in der Produktion.

```swift
import FastCommentsSwift

let userData = SimpleSSOUserData(
    username: "Jane Doe",
    email: "jane@example.com",
    avatar: "https://example.com/avatar.jpg"
)
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
let token = try? sso.prepareToSend()

let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page-1",
    sso: token
)
let sdk = FastCommentsSDK(config: config)
```

`SimpleSSOUserData` unterstützt auch optionale Felder:

- `id` -- Benutzer-ID (standardmäßig die E-Mail, falls nicht gesetzt)
- `displayName` -- separater Anzeigename
- `displayLabel` -- benutzerdefiniertes Label neben dem Namen (z. B. "VIP")
- `websiteUrl` -- Link am Benutzernamen
- `locale` -- Gebietsschema-Code
- `isProfileActivityPrivate` -- Profilaktivität ausblenden (standardmäßig true)

### Sicheres SSO

In der Produktion generiert Ihr Backend ein signiertes SSO-Token mit Ihrem API-Secret. Die iOS-App ruft dieses Token von Ihrem Server ab und übergibt es an die Konfiguration.

**Auf Ihrem Backend** (mit dem FastComments Swift SDK oder einer beliebigen Sprache):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Geben Sie dieses Token über Ihre API an Ihre iOS-App zurück
```

**In Ihrer iOS-App:**

```swift
struct MyView: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-page-1"
        )
    )
    @State private var isLoadingToken = true

    var body: some View {
        Group {
            if isLoadingToken {
                ProgressView("Loading...")
            } else {
                FastCommentsView(sdk: sdk)
            }
        }
        .task {
            // Token von Ihrem Backend abrufen
            let token = try? await fetchSSOTokenFromYourBackend()
            // Neue Konfiguration mit dem Token erstellen oder vor dem Laden setzen
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` unterstützt zusätzliche Felder:

- `optedInNotifications` -- E-Mail-Benachrichtigungs-Anmeldung
- `displayLabel` -- benutzerdefiniertes Label
- `displayName` -- Anzeigename
- `websiteUrl` -- Website-URL
- `groupIds` -- Gruppenmitgliedschaften
- `isAdmin` -- Admin-Rechte
- `isModerator` -- Moderator-Rechte
- `isProfileActivityPrivate` -- Profil-Privatsphäre

---
---