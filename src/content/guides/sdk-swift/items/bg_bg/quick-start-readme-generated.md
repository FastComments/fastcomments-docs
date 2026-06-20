### Използване на публичното API

```swift
import FastCommentsSwift

// Създаване на API клиент
let publicApi = PublicAPI()

// Извличане на коментари за страница
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

### Използване на удостовереното API

```swift
import FastCommentsSwift

// Създаване на конфигурация с API ключ
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Извличане на коментари, използвайки удостовереното API
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

### Използване на API за модерация

```swift
import FastCommentsSwift

// Методите за модерация се упълномощават с `sso` токен за действащия модератор
// (генерирайте го с FastCommentsSSO, вижте раздела за SSO по-горе).
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

### Използване на SSO за удостоверяване

#### Сигурен SSO (Препоръчително за продукция)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Създаване на сигурни SSO данни за потребителя (само на сървъра!)
let userData = SecureSSOUserData(
    id: "user-123",              // Идентификатор на потребителя
    email: "user@example.com",   // Имейл
    username: "johndoe",         // Потребителско име
    avatar: "https://example.com/avatar.jpg" // URL на аватар
)

// Генериране на SSO токен
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Предайте този токен на фронтенда си за удостоверяване
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Прост SSO (За разработка/тестване)

```swift
import FastCommentsSwift

// Създаване на прост SSO потребителски данни (не е необходим API ключ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Генериране на прост SSO токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```