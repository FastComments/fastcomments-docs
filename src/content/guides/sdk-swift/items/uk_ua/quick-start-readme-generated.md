### Використання публічного API

```swift
import FastCommentsSwift

// Отримати коментарі для сторінки
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

### Використання автентифікованого API

```swift
import FastCommentsSwift

// Налаштуйте ваш API-ключ у спільній конфігурації (надсилається у заголовку x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Отримати коментарі, використовуючи автентифікований API
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

### Використання API модерації

```swift
import FastCommentsSwift

// Методи модерації авторизуються за допомогою токену `sso` для діючого модератора
// (згенеруйте його за допомогою FastCommentsSSO, дивіться розділ SSO вище).
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

### Використання SSO для автентифікації

#### Безпечний SSO (рекомендовано для продакшн)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Створіть безпечні дані користувача SSO (лише на сервері!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID користувача
    email: "user@example.com",   // Електронна пошта
    username: "johndoe",         // Ім'я користувача
    avatar: "https://example.com/avatar.jpg" // URL аватара
)

// Створіть SSO токен
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Передайте цей токен вашому фронтенду для автентифікації
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Простий SSO (для розробки/тестування)

```swift
import FastCommentsSwift

// Створіть прості дані користувача SSO (не потрібен API-ключ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Створіть простий SSO токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```