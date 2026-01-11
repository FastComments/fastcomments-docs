### Verwendung der öffentlichen API

```swift
import FastCommentsSwift

// API-Client erstellen
let publicApi = PublicAPI()

// Kommentare für eine Seite abrufen
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

### Verwendung der authentifizierten API

```swift
import FastCommentsSwift

// Konfiguration mit API-Schlüssel erstellen
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Kommentare mit der authentifizierten API abrufen
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

### SSO für die Authentifizierung verwenden

#### Sicheres SSO (Für den Produktiveinsatz empfohlen)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Sichere SSO-Benutzerdaten erstellen (nur serverseitig!)
let userData = SecureSSOUserData(
    id: "user-123",              // Benutzer-ID
    email: "user@example.com",   // E-Mail
    username: "johndoe",         // Benutzername
    avatar: "https://example.com/avatar.jpg" // Avatar-URL
)

// SSO-Token generieren
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Übergeben Sie dieses Token an Ihr Frontend zur Authentifizierung
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Einfaches SSO (Für Entwicklung/Tests)

```swift
import FastCommentsSwift

// Einfache SSO-Benutzerdaten erstellen (kein API-Schlüssel erforderlich)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Einfaches SSO-Token generieren
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```