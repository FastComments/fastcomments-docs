### Public API'yi Kullanma

```swift
import FastCommentsSwift

// Bir sayfa için yorumları al
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

### Kimlik Doğrulamalı API'yi Kullanma

```swift
import FastCommentsSwift

// Paylaşılan yapılandırmada API anahtarınızı ayarlayın (x-api-key başlığı olarak gönderilir)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Kimlik doğrulamalı API kullanarak yorumları al
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

### Moderasyon API'sini Kullanma

```swift
import FastCommentsSwift

// Moderasyon yöntemleri, işlem yapan moderatör için bir `sso` tokenı ile yetkilendirilir
// (FastCommentsSSO ile oluşturun, yukarıdaki SSO bölümüne bakın).
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

### Kimlik Doğrulama İçin SSO Kullanma

#### Güvenli SSO (Üretim İçin Önerilir)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Güvenli SSO kullanıcı verilerini oluştur (sadece sunucu tarafında!)
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
    // Bu tokenı ön ucunuza kimlik doğrulama için gönderin
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Basit SSO (Geliştirme/Test İçin)

```swift
import FastCommentsSwift

// Basit SSO kullanıcı verilerini oluştur (API anahtarı gerekmez)
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