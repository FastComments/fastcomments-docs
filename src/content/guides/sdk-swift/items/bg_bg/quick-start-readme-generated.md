### Използване на публичния API

```swift
import FastCommentsSwift

// Получаване на коментари за страница
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

### Използване на удостоверения API

```swift
import FastCommentsSwift

// Конфигурирайте вашия API ключ в споделената конфигурация (изпраща се като заглавка x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Получаване на коментари чрез автентикиран API
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

### Използване на API за модерация

```swift
import FastCommentsSwift

// Методите за модерация са упълномощени с `sso` токен за изпълняващия ролята модератор
// (генерирайте го с FastCommentsSSO, вижте секцията SSO по-горе).
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

### Използване на SSO за удостоверяване

#### Сигурен SSO (препоръчва се за продукция)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Създаване на сигурни SSO потребителски данни (само от сървъра!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID на потребителя
    email: "user@example.com",   // Имейл
    username: "johndoe",         // Потребителско име
    avatar: "https://example.com/avatar.jpg" // URL на аватара
)

// Генериране на SSO токен
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Предайте този токен към вашия фронтенд за удостоверяване
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Опростен SSO (за разработка/тестове)

```swift
import FastCommentsSwift

// Създаване на опростени SSO потребителски данни (не е нужен API ключ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Генериране на опростен SSO токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```