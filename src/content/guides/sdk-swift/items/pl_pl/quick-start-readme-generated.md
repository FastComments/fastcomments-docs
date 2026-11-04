### Korzystanie z publicznego API

```swift
import FastCommentsSwift

// Pobierz komentarze dla strony
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

### Korzystanie z uwierzytelnionego API

```swift
import FastCommentsSwift

// Skonfiguruj swój klucz API we współdzielonej konfiguracji (wysyłany jako nagłówek x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Pobierz komentarze za pomocą uwierzytelnionego API
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

### Korzystanie z API moderacji

```swift
import FastCommentsSwift

// Metody moderacji są autoryzowane tokenem `sso` działającego moderatora
// (wygeneruj go za pomocą FastCommentsSSO, zobacz sekcję SSO powyżej).
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

### Korzystanie z SSO do uwierzytelniania

#### Bezpieczne SSO (zalecane dla produkcji)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Utwórz bezpieczne dane użytkownika SSO (tylko po stronie serwera!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID użytkownika
    email: "user@example.com",   // Email
    username: "johndoe",         // Nazwa użytkownika
    avatar: "https://example.com/avatar.jpg" // URL awatara
)

// Wygeneruj token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Przekaż ten token do frontendu w celu uwierzytelnienia
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Proste SSO (do rozwoju/testowania)

```swift
import FastCommentsSwift

// Utwórz proste dane użytkownika SSO (nie wymaga klucza API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Wygeneruj prosty token SSO
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```