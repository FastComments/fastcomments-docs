### Коришћење јавног API-ја

```swift
import FastCommentsSwift

// Направите API клијент
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

### Коришћење аутентификованог API-ја

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

### Коришћење SSO за аутентификацију

#### Сигурни SSO (Препоручено за продукцију)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Креирајте сигурне SSO податке о кориснику (само на серверу!)
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

// Креирајте једноставне SSO податке о кориснику (API кључ није потребан)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Генеришите једноставни SSO токен
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```