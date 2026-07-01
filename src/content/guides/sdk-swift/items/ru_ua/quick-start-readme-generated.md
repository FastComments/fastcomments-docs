### Использование публичного API

```swift
import FastCommentsSwift

// Получить комментарии для страницы
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

### Использование аутентифицированного API

```swift
import FastCommentsSwift

// Настройте ваш API‑ключ в общей конфигурации (отправляется в заголовке x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Получить комментарии с помощью аутентифицированного API
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

### Использование API модерации

```swift
import FastCommentsSwift

// Методы модерации авторизованы токеном `sso` от лица модератора
// (создайте его с помощью FastCommentsSSO, см. раздел SSO выше).
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

### Использование SSO для аутентификации

#### Защищённый SSO (рекомендовано для продакшн)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Создать безопасные данные пользователя SSO (только на сервере!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID пользователя
    email: "user@example.com",   // Email
    username: "johndoe",         // Имя пользователя
    avatar: "https://example.com/avatar.jpg" // URL аватара
)

// Сгенерировать SSO‑токен
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Передайте этот токен вашему фронтенду для аутентификации
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Простой SSO (для разработки/тестирования)

```swift
import FastCommentsSwift

// Создать простые данные пользователя SSO (API‑ключ не требуется)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Сгенерировать простой SSO‑токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```