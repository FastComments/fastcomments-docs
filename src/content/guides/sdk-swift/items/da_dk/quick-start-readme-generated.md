### Brug af den offentlige API

```swift
import FastCommentsSwift

// Hent kommentarer for en side
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

### Brug af den autentificerede API

```swift
import FastCommentsSwift

// Konfigurer din API-nøgle i den delte konfiguration (sendt som x-api-key-headeren)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Hent kommentarer ved brug af den autentificerede API
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

### Brug af Moderations-API

```swift
import FastCommentsSwift

// Moderationsmetoder er autoriseret med en `sso`-token for den handlende moderator
// (generer den med FastCommentsSSO, se SSO-sektionen ovenfor).
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

### Brug af SSO til autentificering

#### Sikker SSO (Anbefalet til produktion)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Opret sikker SSO-brugerdata (kun server-side!)
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
    // Send denne token til din frontend for autentificering
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Simpel SSO (Til udvikling/testning)

```swift
import FastCommentsSwift

// Opret simpel SSO-brugerdata (ingen API-nøgle nødvendig)
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