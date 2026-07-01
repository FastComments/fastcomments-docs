### Using the Public API

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

### Using the Authenticated API

```swift
import FastCommentsSwift

// Skonfiguruj swój klucz API w udostępnionej konfiguracji (wysyłany jako nagłówek x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Pobierz komentarze przy użyciu uwierzytelnionego API
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

### Using the Moderation API

```swift
import FastCommentsSwift

// Metody moderacji są autoryzowane tokenem `sso` dla działającego moderatora
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

### Using SSO for Authentication

#### Secure SSO (Recommended for Production)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Utwórz bezpieczne dane użytkownika SSO (tylko po stronie serwera!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID użytkownika
    email: "user@example.com",   // E-mail
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

#### Simple SSO (For Development/Testing)

```swift
import FastCommentsSwift

// Utwórz proste dane użytkownika SSO (klucz API nie jest potrzebny)
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