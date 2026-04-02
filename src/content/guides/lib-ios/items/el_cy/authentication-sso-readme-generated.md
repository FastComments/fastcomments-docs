FastComments υποστηρίζει τρεις τρόπους αυθεντικοποίησης:

1. **Anonymous** -- χωρίς SSO token; οι χρήστες λαμβάνουν ταυτότητες βάσει συνεδρίας
2. **Simple SSO** -- token στην πλευρά του πελάτη για επιδείξεις και δοκιμές (μη ασφαλές)
3. **Secure SSO** -- token υπογεγραμμένο από διακομιστή για παραγωγή

### Simple SSO

Χρήσιμο για επιδείξεις και τοπικές δοκιμές. Οποιοσδήποτε μπορεί να μιμηθεί οποιονδήποτε χρήστη με το Simple SSO, οπότε μην το χρησιμοποιείτε σε παραγωγή.

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

Σε παραγωγή, το backend σας δημιουργεί ένα υπογεγραμμένο SSO token χρησιμοποιώντας το μυστικό API σας. Η εφαρμογή iOS ανακτά αυτό το token από τον διακομιστή σας και το περνά στη διαμόρφωση.

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
            // Ανάκτηση του token από το backend σας
            let token = try? await fetchSSOTokenFromYourBackend()
            // Δημιουργήστε μια νέα διαμόρφωση με το token, ή ορίστε το πριν τη φόρτωση
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