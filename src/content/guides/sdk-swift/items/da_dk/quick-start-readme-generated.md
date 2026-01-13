### Brug af den offentlige API

```swift
import FastCommentsSwift

// Opret API-klient
let publicApi = PublicAPI()

// Hent kommentarer for en side
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

### Brug af den autentificerede API

```swift
import FastCommentsSwift

// Opret konfiguration med API-nøgle
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Hent kommentarer ved hjælp af den autentificerede API
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

### Brug af SSO til autentificering

#### Sikker SSO (Anbefales til produktion)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Opret sikker SSO-brugerdata (kun på serversiden!)
let userData = SecureSSOUserData(
    id: "user-123",              // Bruger-ID
    email: "user@example.com",   // E-mail
    username: "johndoe",         // Brugernavn
    avatar: "https://example.com/avatar.jpg" // Avatar-URL
)

// Generer SSO-token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Send denne token til din frontend til autentificering
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Simpel SSO (Til udvikling/test)

```swift
import FastCommentsSwift

// Opret simple SSO-brugerdata (ingen API-nøgle nødvendig)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generer simpel SSO-token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```