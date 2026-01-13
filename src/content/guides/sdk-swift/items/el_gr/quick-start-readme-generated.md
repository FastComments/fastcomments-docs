### Χρήση του Δημόσιου API

```swift
import FastCommentsSwift

// Δημιουργία πελάτη API
let publicApi = PublicAPI()

// Ανάκτηση σχολίων για μια σελίδα
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

### Χρήση του Αυθεντικοποιημένου API

```swift
import FastCommentsSwift

// Δημιουργία διαμόρφωσης με κλειδί API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Ανάκτηση σχολίων χρησιμοποιώντας το αυθεντικοποιημένο API
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

### Χρήση SSO για αυθεντικοποίηση

#### Ασφαλές SSO (Συνιστάται για Παραγωγή)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Δημιουργία ασφαλών δεδομένων χρήστη SSO (μόνο από τον διακομιστή!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID χρήστη
    email: "user@example.com",   // Email
    username: "johndoe",         // Όνομα χρήστη
    avatar: "https://example.com/avatar.jpg" // URL εικόνας προφίλ
)

// Δημιουργία token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Περάστε αυτό το token στο frontend σας για αυθεντικοποίηση
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Απλό SSO (Για ανάπτυξη/δοκιμές)

```swift
import FastCommentsSwift

// Δημιουργία απλών δεδομένων χρήστη SSO (δεν απαιτείται κλειδί API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Δημιουργία απλού token SSO
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```
---