### Использование публичного API

```swift
import FastCommentsSwift

// Создать API-клиент
let publicApi = PublicAPI()

// Fetch comments for a page
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

### Использование аутентифицированного API

```swift
import FastCommentsSwift

// Create configuration with API key
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Fetch comments using authenticated API
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

### Использование SSO для аутентификации

#### Защищённый SSO (Рекомендуется для производственной среды)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Создать защищённые данные пользователя SSO (только на стороне сервера!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID пользователя
    email: "user@example.com",   // Электронная почта
    username: "johndoe",         // Имя пользователя
    avatar: "https://example.com/avatar.jpg" // URL аватара
)

// Generate SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Передайте этот токен на фронтенд для аутентификации
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Простой SSO (для разработки/тестирования)

```swift
import FastCommentsSwift

// Создать простые данные пользователя SSO (API-ключ не требуется)
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