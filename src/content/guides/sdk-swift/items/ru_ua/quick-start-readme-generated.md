### Использование публичного API

```swift
import FastCommentsSwift

// Создать клиент API
let publicApi = PublicAPI()

// Получить комментарии для страницы
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

// Создать конфигурацию с API-ключом
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Получить комментарии, используя аутентифицированный API
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

### Использование API модерации

```swift
import FastCommentsSwift

// Методы модерации авторизуются с помощью токена `sso` для действующего модератора
// (сгенерируйте его с помощью FastCommentsSSO, см. раздел SSO выше).
do {
    let response = try await ModerationAPI.getApiComments(
        page: 0,
        count: 30,
        sso: ssoToken
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

#### Secure SSO (Рекомендуется для продакшена)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Создать данные пользователя для защищённого SSO (только на сервере!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID пользователя
    email: "user@example.com",   // Электронная почта
    username: "johndoe",         // Имя пользователя
    avatar: "https://example.com/avatar.jpg" // URL аватара
)

// Сгенерировать SSO-токен
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Передайте этот токен на фронтенд для аутентификации
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Simple SSO (Для разработки/тестирования)

```swift
import FastCommentsSwift

// Создать данные пользователя для простого SSO (API-ключ не требуется)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Сгенерировать простой SSO-токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```