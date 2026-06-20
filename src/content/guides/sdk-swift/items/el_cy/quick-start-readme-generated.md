### Χρήση του Δημόσιου API

```swift
import FastCommentsSwift

// Δημιούργησε πελάτη API
let publicApi = PublicAPI()

// Ανάκτησε σχόλια για μια σελίδα
do {
    let response = try await publicApi.getCommentsPublic(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Found \(response.comments?.count ?? 0) comments")
    for comment in response.comments ?? [] {
        print("Comment: \(comment.comment ?? "")")
    }
} catch {
    print("Error fetching comments: \(error)")
}
```

### Χρήση του API με Αυθεντικοποίηση

```swift
import FastCommentsSwift

// Δημιούργησε ρύθμιση με κλειδί API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Ανάκτησε σχόλια χρησιμοποιώντας το αυθεντικοποιημένο API
do {
    let response = try await defaultApi.getComments(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Total comments: \(response.count ?? 0)")
    for comment in response.comments ?? [] {
        print("Comment ID: \(comment.id ?? ""), Text: \(comment.comment ?? "")")
    }
} catch {
    print("Error: \(error)")
}
```

### Χρήση του API Εποπτείας

```swift
import FastCommentsSwift

// Οι μέθοδοι εποπτείας εξουσιοδοτούνται με ένα `sso` token για τον λειτουργούντα συντονιστή
// (παράγεται με το FastCommentsSSO, δείτε την ενότητα SSO πιο πάνω).
do {
    let response = try await ModerationAPI.getApiComments(
        page: 0,
        count: 30,
        sso: ssoToken
    )

    print("Found \(response.comments.count) comments to moderate")
    for comment in response.comments {
        print("Comment ID: \(comment.id), Text: \(comment.commentHTML)")
    }
} catch {
    print("Error: \(error)")
}
```

### Χρήση SSO για Αυθεντικοποίηση

#### Ασφαλές SSO (Συνιστάται για Παραγωγή)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Δημιούργησε ασφαλή δεδομένα χρήστη SSO (μόνο πλευρά διακομιστή!)
let userData = SecureSSOUserData(
    id: "user-123",              // Αναγνωριστικό χρήστη
    email: "user@example.com",   // Ηλεκτρονικό ταχυδρομείο
    username: "johndoe",         // Όνομα χρήστη
    avatar: "https://example.com/avatar.jpg" // URL Άβαταρ
)

// Δημιούργησε token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Πέρασε αυτό το token στο frontend σου για αυθεντικοποίηση
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Απλό SSO (Για Ανάπτυξη/Δοκιμές)

```swift
import FastCommentsSwift

// Δημιούργησε απλά δεδομένα χρήστη SSO (δεν χρειάζεται κλειδί API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Δημιούργησε απλό token SSO
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```