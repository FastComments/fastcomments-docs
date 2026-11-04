Το FastComments υποστηρίζει τρεις τρόπους ελέγχου ταυτότητας:

1. **Ανώνυμος** -- χωρίς SSO token· οι χρήστες αποκτούν ταυτότητες βασισμένες σε συνεδρία
2. **Απλό SSO** -- token από την πλευρά του πελάτη για demos και δοκιμές (δεν είναι ασφαλές)
3. **Ασφαλές SSO** -- token υπογεγραμμένο από τον διακομιστή για παραγωγή

### Απλό SSO

Χρήσιμο για demos και τοπικές δοκιμές. Με το Απλό SSO οποιοσδήποτε μπορεί να υποδυθεί οποιονδήποτε χρήστη, οπότε μην το χρησιμοποιείτε σε περιβάλλον παραγωγής.

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

Το `SimpleSSOUserData` υποστηρίζει επίσης προαιρετικά πεδία:

- `id` -- ID χρήστη (προεπιλέγεται στο email αν δεν οριστεί)
- `displayName` -- ξεχωριστό εμφανιζόμενο όνομα
- `displayLabel` -- προσαρμοσμένη ετικέτα που εμφανίζεται δίπλα στο όνομα (π.χ. "VIP")
- `websiteUrl` -- σύνδεσμος στο όνομα του χρήστη
- `locale` -- κωδικός τοπικών ρυθμίσεων
- `isProfileActivityPrivate` -- απόκρυψη δραστηριότητας προφίλ (προεπιλογή: true)

### Ασφαλές SSO

Σε περιβάλλον παραγωγής, το backend σας δημιουργεί ένα υπογεγραμμένο SSO token χρησιμοποιώντας το API secret σας. Η εφαρμογή iOS ανακτά αυτό το token από τον διακομιστή σας και το περνά στη διαμόρφωση.

**Στο backend σας** (χρησιμοποιώντας το FastComments Swift SDK ή οποιαδήποτε γλώσσα):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Επιστρέψτε αυτό το token στην εφαρμογή iOS σας μέσω του API σας
```

**Στην εφαρμογή iOS σας:**

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
            // Δημιουργήστε μια νέα διαμόρφωση με το token, ή ορίστε το πριν τη φόρτωση
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

Το `SecureSSOUserData` υποστηρίζει επιπλέον πεδία:

- `optedInNotifications` -- επιλογή λήψης ειδοποιήσεων email
- `displayLabel` -- προσαρμοσμένη ετικέτα
- `displayName` -- εμφανιζόμενο όνομα
- `websiteUrl` -- URL ιστοσελίδας
- `groupIds` -- συμμετοχές σε ομάδες
- `isAdmin` -- δικαιώματα διαχειριστή
- `isModerator` -- δικαιώματα συντονιστή
- `isProfileActivityPrivate` -- ιδιωτικότητα προφίλ

---