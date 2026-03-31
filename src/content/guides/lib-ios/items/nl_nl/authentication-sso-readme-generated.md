FastComments ondersteunt drie authenticatiemodi:

1. **Anonymous** -- geen SSO-token; gebruikers krijgen op sessies gebaseerde identiteiten
2. **Simple SSO** -- token aan de clientzijde voor demo's en testen (niet veilig)
3. **Secure SSO** -- door de server ondertekend token voor productie

### Simple SSO

Handig voor demo's en lokaal testen. Iedereen kan met Simple SSO zich voordoen als elke gebruiker, dus gebruik het niet in productie.

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

- `id` -- gebruikers-ID (standaard het e-mailadres als dit niet is ingesteld)
- `displayName` -- aparte weergavenaam
- `displayLabel` -- aangepast label dat naast de naam wordt weergegeven (bijv. "VIP")
- `websiteUrl` -- link op de gebruikersnaam
- `locale` -- locale-code
- `isProfileActivityPrivate` -- verberg profielactiviteit (standaard: true)

### Secure SSO

In productie genereert je backend een door de server ondertekend SSO-token met je API-secret. De iOS-app haalt dit token van je server en geeft het door aan de configuratie.

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
// Return this token to your iOS app via your API
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
            // Fetch the token from your backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Create a new config with the token, or set it before load
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` ondersteunt extra velden:

- `optedInNotifications` -- opt-in voor e-mailmeldingen
- `displayLabel` -- aangepast label
- `displayName` -- weergavenaam
- `websiteUrl` -- website-URL
- `groupIds` -- groepslidmaatschappen
- `isAdmin` -- adminrechten
- `isModerator` -- moderatorrechten
- `isProfileActivityPrivate` -- profielprivacy

---
---