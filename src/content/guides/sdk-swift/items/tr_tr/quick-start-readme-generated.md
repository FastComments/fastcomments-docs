### Herkese Açık API'yi Kullanma

```swift
import FastCommentsSwift

// API istemcisi oluştur
let publicApi = PublicAPI()

// Bir sayfa için yorumları al
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

### Kimlik Doğrulamalı API'yi Kullanma

```swift
import FastCommentsSwift

// API anahtarı ile yapılandırma oluştur
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Kimlik doğrulamalı API kullanarak yorumları al
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

### Kimlik Doğrulama için SSO Kullanımı

#### Güvenli SSO (Prodüksiyon için önerilir)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Güvenli SSO kullanıcı verisi oluştur (sadece sunucu tarafında!)
let userData = SecureSSOUserData(
    id: "user-123",              // Kullanıcı ID'si
    email: "user@example.com",   // E-posta
    username: "johndoe",         // Kullanıcı adı
    avatar: "https://example.com/avatar.jpg" // Avatar URL'si
)

// SSO belirteci (token) oluştur
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Bu belirteci kimlik doğrulama için ön uca iletin
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Basit SSO (Geliştirme/Test için)

```swift
import FastCommentsSwift

// Basit SSO kullanıcı verisi oluştur (API anahtarı gerekmez)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Basit SSO belirteci oluştur
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```