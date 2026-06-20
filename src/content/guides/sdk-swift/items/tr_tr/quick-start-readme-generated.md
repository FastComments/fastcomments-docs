### Public API'yi Kullanma

```swift
import FastCommentsSwift

// API istemcisi oluştur
let publicApi = PublicAPI()

// Bir sayfa için yorumları getir
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

### Yetkilendirilmiş API'yi Kullanma

```swift
import FastCommentsSwift

// API anahtarıyla yapılandırma oluştur
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Kimlik doğrulanmış API'yi kullanarak yorumları getir
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

### Moderasyon API'sini Kullanma

```swift
import FastCommentsSwift

// Moderasyon yöntemleri, görev yapan moderatör için bir `sso` tokeni ile yetkilendirilir
// (bunu FastCommentsSSO ile oluşturun, yukarıdaki SSO bölümüne bakın).
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

### Kimlik Doğrulama için SSO Kullanma

#### Güvenli SSO (Üretim için Önerilir)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Güvenli SSO kullanıcı verisi oluşturun (yalnızca sunucu tarafı!)
let userData = SecureSSOUserData(
    id: "user-123",              // Kullanıcı Kimliği
    email: "user@example.com",   // E-posta
    username: "johndoe",         // Kullanıcı adı
    avatar: "https://example.com/avatar.jpg" // Avatar URL
)

// SSO tokenı oluştur
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Kimlik doğrulama için bu tokenı frontend'inize iletin
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Basit SSO (Geliştirme/Test için)

```swift
import FastCommentsSwift

// Basit SSO kullanıcı verisi oluştur (API anahtarına gerek yok)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Basit SSO tokenı oluştur
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```