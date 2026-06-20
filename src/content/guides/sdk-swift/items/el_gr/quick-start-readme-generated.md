### Χρήση του Public API

```swift
import FastCommentsSwift

// Δημιουργία client API
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

### Χρήση του Πιστοποιημένου API

```swift
import FastCommentsSwift

// Δημιουργία ρύθμισης με κλειδί API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Ανάκτηση σχολίων χρησιμοποιώντας πιστοποιημένο API
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

### Χρήση του Moderation API

```swift
import FastCommentsSwift

// Οι μέθοδοι moderation εξουσιοδοτούνται με ένα token `sso` για τον ενεργό moderator
// (παράγετε το με FastCommentsSSO, δείτε την ενότητα SSO παραπάνω).
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

#### Secure SSO (Συνιστάται για Παραγωγή)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Δημιουργία ασφαλών δεδομένων χρήστη SSO (μόνο στο server!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID χρήστη
    email: "user@example.com",   // Email
    username: "johndoe",         // Όνομα χρήστη
    avatar: "https://example.com/avatar.jpg" // URL avatar
)

// Δημιουργία token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Δώστε αυτό το token στο frontend σας για αυθεντικοποίηση
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Simple SSO (Για Ανάπτυξη/Δοκιμές)

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