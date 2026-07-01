### Χρήση του Δημόσιου API

```swift
import FastCommentsSwift

// Ανάκτηση σχολίων για μια σελίδα
do {
    let response = try await PublicAPI.getCommentsPublic(
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

// Διαμόρφωση του κλειδιού API στη κοινή διαμόρφωση (αποστέλλεται ως κεφαλίδα x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Ανάκτηση σχολίων χρησιμοποιώντας το πιστοποιημένο API
do {
    let response = try await DefaultAPI.getComments(
        tenantId: "your-tenant-id",
        options: .init(urlId: "page-url-id")
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

// Οι μέθοδοι διαχείρισης εξουσιοδοτούνται με ένα διακριτικό `sso` για τον ενεργό συντονιστή
// (δημιουργήστε το με FastCommentsSSO, ανατρέξτε στην ενότητα SSO παραπάνω).
do {
    let response = try await ModerationAPI.getApiComments(
        options: .init(
            page: 0,
            count: 30,
            sso: ssoToken
        )
    )

    print("Found \(response.comments.count) comments to moderate")
    for comment in response.comments {
        print("Comment ID: \(comment.id), Text: \(comment.commentHTML)")
    }
} catch {
    print("Error: \(error)")
}
```

### Χρήση του SSO για Έλεγχο Ταυτότητας

#### Ασφαλές SSO (Προτείνεται για Παραγωγή)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Δημιουργία ασφαλών δεδομένων χρήστη SSO (μόνο στο διακομιστή!)
let userData = SecureSSOUserData(
    id: "user-123",              // Αναγνωριστικό χρήστη
    email: "user@example.com",   // Email
    username: "johndoe",         // Όνομα χρήστη
    avatar: "https://example.com/avatar.jpg" // URL Αφικής Εικόνας
)

// Δημιουργία διακριτικού SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Περνάτε αυτό το διακριτικό στο frontend σας για έλεγχο ταυτότητας
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Απλό SSO (Για Ανάπτυξη/Δοκιμή)

```swift
import FastCommentsSwift

// Δημιουργία απλών δεδομένων χρήστη SSO (δεν απαιτείται κλειδί API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Δημιουργία απλού διακριτικού SSO
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```