FastComments supports three authentication modes:

1. **Anonymous** -- χωρίς SSO token; οι χρήστες λαμβάνουν ταυτότητες με βάση τη συνεδρία
2. **Simple SSO** -- token στην πλευρά του πελάτη για παρουσιάσεις και τοπικές δοκιμές (μη ασφαλές)
3. **Secure SSO** -- token υπογεγραμμένο από διακομιστή για παραγωγή

### Simple SSO

Χρήσιμο για παρουσιάσεις και τοπικές δοκιμές. Ο οποιοσδήποτε μπορεί να προσποιηθεί οποιονδήποτε χρήστη με το Simple SSO, οπότε μην το χρησιμοποιείτε σε παραγωγή.

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

- `id` -- user ID (defaults to email if not set) -> `id` -- αναγνωριστικό χρήστη (εξ ορισμού το email αν δεν οριστεί)
- `displayName` -- separate display name -> `displayName` -- ξεχωριστό εμφανιζόμενο όνομα
- `displayLabel` -- custom label shown next to the name (e.g. "VIP") -> `displayLabel` -- προσαρμοσμένη ετικέτα που εμφανίζεται δίπλα στο όνομα (π.χ. "VIP")
- `websiteUrl` -- link on the user's name -> `websiteUrl` -- σύνδεσμος στο όνομα του χρήστη
- `locale` -- locale code -> `locale` -- κωδικός τοπικής ρύθμισης
- `isProfileActivityPrivate` -- hide profile activity (defaults to true) -> `isProfileActivityPrivate` -- απόκρυψη δραστηριότητας προφίλ (εξ ορισμού true)

### Secure SSO

Σε παραγωγή, το backend σας δημιουργεί ένα υπογεγραμμένο SSO token χρησιμοποιώντας το API secret σας. Η εφαρμογή iOS ανακτά αυτό το token από τον διακομιστή σας και το περνάει στο config.

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
// Επιστρέψτε αυτό το token στην εφαρμογή iOS μέσω του API σας
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
            // Ανακτήστε το token από το backend σας
            let token = try? await fetchSSOTokenFromYourBackend()
            // Δημιουργήστε ένα νέο config με το token, ή ορίστε το πριν το φόρτωμα
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` supports additional fields:

- `optedInNotifications` -- email notification opt-in -> `optedInNotifications` -- επιλογή λήψης ειδοποιήσεων μέσω email
- `displayLabel` -- custom label -> `displayLabel` -- προσαρμοσμένη ετικέτα
- `displayName` -- display name -> `displayName` -- εμφανιζόμενο όνομα
- `websiteUrl` -- website URL -> `websiteUrl` -- URL ιστότοπου
- `groupIds` -- group memberships -> `groupIds` -- συμμετοχές σε ομάδες
- `isAdmin` -- admin privileges -> `isAdmin` -- προνόμια διαχειριστή
- `isModerator` -- moderator privileges -> `isModerator` -- προνόμια συντονιστή
- `isProfileActivityPrivate` -- profile privacy -> `isProfileActivityPrivate` -- ιδιωτικότητα δραστηριότητας προφίλ

---
---