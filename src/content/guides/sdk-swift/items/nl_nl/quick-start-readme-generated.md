### Gebruik van de openbare API

```swift
import FastCommentsSwift

// Maak API-client
let publicApi = PublicAPI()

// Haal reacties op voor een pagina
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

### Gebruik van de geauthenticeerde API

```swift
import FastCommentsSwift

// Maak configuratie met API-sleutel
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Haal reacties op met de geauthenticeerde API
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

### SSO gebruiken voor authenticatie

#### Beveiligde SSO (Aanbevolen voor productie)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Maak beveiligde SSO-gebruikergegevens (alleen server-side!)
let userData = SecureSSOUserData(
    id: "user-123",              // Gebruikers-ID
    email: "user@example.com",   // E-mail
    username: "johndoe",         // Gebruikersnaam
    avatar: "https://example.com/avatar.jpg" // Avatar-URL
)

// Genereer SSO-token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Geef dit token door aan je frontend voor authenticatie
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Eenvoudige SSO (Voor ontwikkeling/testen)

```swift
import FastCommentsSwift

// Maak eenvoudige SSO-gebruikergegevens (geen API-sleutel nodig)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Genereer eenvoudig SSO-token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```