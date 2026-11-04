FastComments ondersteunt drie authenticatiemodi:

1. **Anoniem** -- geen SSO-token; gebruikers krijgen sessiegebaseerde identiteiten
2. **Simple SSO** -- client-side token voor demo's en testen (niet veilig)
3. **Secure SSO** -- server-ondertekend token voor productie

### Simple SSO

Handig voor demo's en lokaal testen. Met Simple SSO kan iedereen zich voordoen als elke willekeurige gebruiker, dus gebruik het niet in productie.

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

`SimpleSSOUserData` ondersteunt ook optionele velden:

- `id` -- gebruikers-ID (standaard het e-mailadres indien niet ingesteld)
- `displayName` -- aparte weergavenaam
- `displayLabel` -- aangepast label naast de naam (bijv. "VIP")
- `websiteUrl` -- link op de naam van de gebruiker
- `locale` -- locale-code
- `isProfileActivityPrivate` -- profielactiviteit verbergen (standaard true)

### Secure SSO

In productie genereert je backend een ondertekend SSO-token met behulp van je API-secret. De iOS-app haalt dit token op bij je server en geeft het door aan de configuratie.

**Op je backend** (met de FastComments Swift SDK of een andere taal):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Geef dit token terug aan je iOS-app via je API
```

**In je iOS-app:**

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
            // Haal het token op bij je backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Maak een nieuwe configuratie met het token, of stel het in vóór het laden
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` ondersteunt aanvullende velden:

- `optedInNotifications` -- aanmelding voor e-mailnotificaties
- `displayLabel` -- aangepast label
- `displayName` -- weergavenaam
- `websiteUrl` -- website-URL
- `groupIds` -- groepslidmaatschappen
- `isAdmin` -- beheerdersrechten
- `isModerator` -- moderatierechten
- `isProfileActivityPrivate` -- profielprivacy

---
---