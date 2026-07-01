### Gebruik van de openbare API

```swift
import FastCommentsSwift

// Haal reacties op voor een pagina
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

### Gebruik van de geauthenticeerde API

```swift
import FastCommentsSwift

// Stel uw API-sleutel in op de gedeelde configuratie (verzonden als de x-api-key header)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Haal reacties op met geauthentiseerde API
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

### Gebruik van de moderatie-API

```swift
import FastCommentsSwift

// Moderatiemethoden zijn geautoriseerd met een `sso`-token voor de optredende moderator
// (genereer het met FastCommentsSSO, zie de SSO sectie hierboven).
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

### Gebruik van SSO voor authenticatie

#### Veilige SSO (Aanbevolen voor productie)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Maak veilige SSO-gebruikersgegevens aan (alleen server‑side!)
let userData = SecureSSOUserData(
    id: "user-123",              // Gebruikers-ID
    email: "user@example.com",   // E‑mail
    username: "johndoe",         // Gebruikersnaam
    avatar: "https://example.com/avatar.jpg" // Avatar‑URL
)

// Genereer SSO-token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Stuur dit token naar uw front‑end voor authenticatie
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Eenvoudige SSO (Voor ontwikkeling/testen)

```swift
import FastCommentsSwift

// Maak eenvoudige SSO-gebruikersgegevens aan (geen API-sleutel nodig)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Genereer eenvoudige SSO-token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```