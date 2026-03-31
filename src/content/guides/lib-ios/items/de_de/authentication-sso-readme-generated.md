FastComments unterstützt drei Authentifizierungsmodi:

1. **Anonymous** -- kein SSO-Token; Benutzer erhalten sitzungsbasierte Identitäten
2. **Simple SSO** -- clientseitiges Token für Demos und Tests (nicht sicher)
3. **Secure SSO** -- server-signiertes Token für den Produktionseinsatz

### Simple SSO

Nützlich für Demos und lokale Tests. Jeder kann mit Simple SSO jeden Benutzer nachahmen, verwenden Sie es daher nicht in Produktionsumgebungen.

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

`SimpleSSOUserData` unterstützt außerdem optionale Felder:

- `id` -- Benutzer-ID (standardmäßig die E-Mail, falls nicht gesetzt)
- `displayName` -- separater Anzeigename
- `displayLabel` -- benutzerdefiniertes Label, das neben dem Namen angezeigt wird (z. B. "VIP")
- `websiteUrl` -- Link beim Namen des Benutzers
- `locale` -- Lokalisierungscode
- `isProfileActivityPrivate` -- Profilaktivität ausblenden (standardmäßig true)

### Secure SSO

In Produktionsumgebungen erzeugt Ihr Backend ein signiertes SSO-Token mithilfe Ihres API-Secrets. Die iOS-App holt dieses Token von Ihrem Server und übergibt es an die Konfiguration.

**On your backend** (using the FastComments Swift SDK or any language):

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
            // Rufen Sie das Token von Ihrem Backend ab
            let token = try? await fetchSSOTokenFromYourBackend()
            // Erstellen Sie eine neue Konfiguration mit dem Token oder setzen Sie es vor dem Laden
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` unterstützt zusätzliche Felder:

- `optedInNotifications` -- Opt-in für E-Mail-Benachrichtigungen
- `displayLabel` -- benutzerdefiniertes Label
- `displayName` -- Anzeigename
- `websiteUrl` -- Website-URL
- `groupIds` -- Gruppenmitgliedschaften
- `isAdmin` -- Admin-Rechte
- `isModerator` -- Moderator-Rechte
- `isProfileActivityPrivate` -- Profil-Datenschutz

---
---