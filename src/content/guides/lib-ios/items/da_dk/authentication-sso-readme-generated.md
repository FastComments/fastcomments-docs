FastComments understøtter tre autentificeringsmetoder:

1. **Anonym** -- ingen SSO-token; brugere får sessionbaserede identiteter
2. **Simpel SSO** -- klientside-token til demoer og test (ikke sikker)
3. **Sikker SSO** -- serversigneret token til produktion

### Simpel SSO

Brugbar til demoer og lokal testning. Alle kan udgive sig for en hvilken som helst bruger med Simpel SSO, så brug det ikke i produktion.

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

`SimpleSSOUserData` understøtter også valgfrie felter:

- `id` -- bruger-id (som standard e-mail hvis ikke angivet)
- `displayName` -- separat visningsnavn
- `displayLabel` -- brugerdefineret label vist ved siden af navnet (f.eks. "VIP")
- `websiteUrl` -- link på brugerens navn
- `locale` -- locale-kode
- `isProfileActivityPrivate` -- skjuler profilaktivitet (som standard true)

### Sikker SSO

I produktion genererer din backend et signerede SSO-token ved hjælp af din API-secret. iOS-appen henter dette token fra din server og sender det til konfigurationen.

**På din backend** (ved brug af FastComments Swift SDK eller ethvert sprog):

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

**I din iOS-app:**

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

`SecureSSOUserData` understøtter yderligere felter:

- `optedInNotifications` -- tilvalg af e-mailnotifikationer
- `displayLabel` -- brugerdefineret label
- `displayName` -- visningsnavn
- `websiteUrl` -- webadresse
- `groupIds` -- gruppe-medlemskaber
- `isAdmin` -- administratorrettigheder
- `isModerator` -- moderatorrettigheder
- `isProfileActivityPrivate` -- profilens privatliv

---
---