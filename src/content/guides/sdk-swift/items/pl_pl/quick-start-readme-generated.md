### Korzystanie z publicznego API

```swift
import FastCommentsSwift

// Utwórz klienta API
let publicApi = PublicAPI()

// Pobierz komentarze dla strony
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

### Korzystanie z uwierzytelnionego API

```swift
import FastCommentsSwift

// Utwórz konfigurację z kluczem API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Pobierz komentarze używając uwierzytelnionego API
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

### Korzystanie z SSO do uwierzytelniania

#### Bezpieczne SSO (zalecane dla produkcji)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Utwórz bezpieczne dane użytkownika SSO (tylko po stronie serwera!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID użytkownika
    email: "user@example.com",   // E-mail
    username: "johndoe",         // Nazwa użytkownika
    avatar: "https://example.com/avatar.jpg" // URL avatara
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

#### Proste SSO (do rozwoju/testów)

```swift
import FastCommentsSwift

// Utwórz proste dane użytkownika SSO (brak potrzeby klucza API)
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