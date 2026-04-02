FastComments supports three authentication modes:

1. **Anonymous** -- no SSO token; users get session-based identities
2. **Simple SSO** -- client-side token for demos and testing (not secure)
3. **Secure SSO** -- server-signed token for production

### Simple SSO

Useful for demos and local testing. Anyone can impersonate any user with Simple SSO, so do not use it in production.

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

### Secure SSO

In production, your backend generates a signed SSO token using your API secret. The iOS app fetches this token from your server and passes it to the config.

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