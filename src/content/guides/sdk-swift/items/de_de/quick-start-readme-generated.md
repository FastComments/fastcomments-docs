### Verwendung der öffentlichen API

```swift
import FastCommentsSwift

// Kommentare für eine Seite abrufen
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

### Verwendung der authentifizierten API

```swift
import FastCommentsSwift

// Konfigurieren Sie Ihren API-Schlüssel in der gemeinsam genutzten Konfiguration (gesendet als x-api-key Header)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Kommentare mit der authentifizierten API abrufen
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

### Verwendung der Moderations-API

```swift
import FastCommentsSwift

// Moderationsmethoden werden mit einem `sso`-Token für den handelnden Moderator autorisiert
// (generieren Sie ihn mit FastCommentsSSO, siehe den SSO-Abschnitt oben).
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

### Verwendung von SSO zur Authentifizierung

#### Sicheres SSO (Empfohlen für die Produktion)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Erstelle sichere SSO-Benutzerdaten (nur serverseitig!)
let userData = SecureSSOUserData(
    id: "user-123",              // Benutzer-ID
    email: "user@example.com",   // E-Mail
    username: "johndoe",         // Benutzername
    avatar: "https://example.com/avatar.jpg" // Avatar-URL
)

// Generate SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Übergib dieses Token an dein Frontend zur Authentifizierung
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Einfaches SSO (Für Entwicklung/Tests)

```swift
import FastCommentsSwift

// Erstelle einfache SSO-Benutzerdaten (kein API-Schlüssel erforderlich)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generate simple SSO token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```