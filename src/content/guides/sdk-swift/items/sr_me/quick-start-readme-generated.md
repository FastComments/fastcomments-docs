### Коришћење Јавног API-ја

```swift
import FastCommentsSwift

// Креирајте API клијент
let publicApi = PublicAPI()

// Преузмите коментаре за страницу
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

### Коришћење Аутентификованог API-ја

```swift
import FastCommentsSwift

// Креирајте конфигурацију са API кључем
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Преузмите коментаре користећи аутентификовани API
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

### Коришћење API-ја за модерацију

```swift
import FastCommentsSwift

// Методе модерације ауторизују се са `sso` токеном за модератора у акцији
// (генеришите га са FastCommentsSSO, погледајте SSO одељак изнад).
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

### Коришћење SSO за аутентификацију

#### Безбедни SSO (Препоручено за производно окружење)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Креирајте безбедне SSO податке корисника (само на серверу!)
let userData = SecureSSOUserData(
    id: "user-123",              // ИД корисника
    email: "user@example.com",   // Е-пошта
    username: "johndoe",         // Корисничко име
    avatar: "https://example.com/avatar.jpg" // URL аватара
)

// Генеришите SSO токен
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Проследите овај токен вашем фронтенду за аутентификацију
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Једноставни SSO (За развој/тестирање)

```swift
import FastCommentsSwift

// Креирајте једноставне SSO податке корисника (није потребан API кључ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Генеришите једноставан SSO токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```