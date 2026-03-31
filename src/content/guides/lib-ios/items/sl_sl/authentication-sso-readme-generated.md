FastComments podpira tri načine preverjanja pristnosti:

1. **Anonimno** -- brez SSO žetona; uporabniki dobijo identitete, vezane na sejo
2. **Preprosti SSO** -- odjemalski žeton za predstavitve in testiranje (ni varen)
3. **Varen SSO** -- strežniško podpisan žeton za produkcijo

### Preprosti SSO

Uporabno za predstavitve in lokalno testiranje. Z Preprostim SSO lahko kdorkoli prevzame identiteto kateregakoli uporabnika, zato ga ne uporabljajte v produkciji.

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

`SimpleSSOUserData` also supports optional fields:

- `id` -- user ID (defaults to email if not set)
- `displayName` -- separate display name
- `displayLabel` -- custom label shown next to the name (e.g. "VIP")
- `websiteUrl` -- link on the user's name
- `locale` -- locale code
- `isProfileActivityPrivate` -- hide profile activity (defaults to true)

### Varen SSO

V v produkciji vaš backend ustvari podpisan SSO žeton z uporabo vašega API tajnega ključa. iOS aplikacija pridobi ta žeton z vašega strežnika in ga posreduje konfiguraciji.

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

**In your iOS app:**

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

`SecureSSOUserData` supports additional fields:

- `optedInNotifications` -- email notification opt-in
- `displayLabel` -- custom label
- `displayName` -- display name
- `websiteUrl` -- website URL
- `groupIds` -- group memberships
- `isAdmin` -- admin privileges
- `isModerator` -- moderator privileges
- `isProfileActivityPrivate` -- profile privacy

---
---