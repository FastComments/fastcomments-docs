### Використання публічного API

```swift
import FastCommentsSwift

// Створити клієнта API
let publicApi = PublicAPI()

// Отримати коментарі для сторінки
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

### Використання автентифікованого API

```swift
import FastCommentsSwift

// Створити конфігурацію з API-ключем
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Отримати коментарі з автентифікованого API
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

### Використання SSO для автентифікації

#### Безпечний SSO (Рекомендується для виробничого середовища)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Створити безпечні дані користувача SSO (тільки на сервері!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID користувача
    email: "user@example.com",   // Електронна пошта
    username: "johndoe",         // Ім'я користувача
    avatar: "https://example.com/avatar.jpg" // URL аватара
)

// Згенерувати SSO токен
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Передайте цей токен на фронтенд для автентифікації
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Простий SSO (Для розробки/тестування)

```swift
import FastCommentsSwift

// Створити прості дані користувача SSO (API-ключ не потрібен)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Згенерувати простий SSO токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```