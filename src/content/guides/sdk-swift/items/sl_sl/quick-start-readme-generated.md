### Uporaba javnega API-ja

```swift
import FastCommentsSwift

// Pridobi komentarje za stran
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

### Uporaba avtenticiranega API-ja

```swift
import FastCommentsSwift

// Nastavite svoj API ključ v deljeni konfiguraciji (poslano kot glava x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Pridobite komentarje z avtenticiranim API-jem
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

### Uporaba Moderacijskega API-ja

```swift
import FastCommentsSwift

// Metode moderiranja so avtorizirane s `sso` žetonom za delujočega moderatorja
// (ustvarite ga s FastCommentsSSO, glejte razdelek SSO zgoraj).
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

### Uporaba SSO za avtentikacijo

#### Varno SSO (priporočeno za produkcijo)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Ustvarite varne SSO uporabniške podatke (samo na strežniku!)
let userData = SecureSSOUserData(
    id: "user-123",              // User ID
    email: "user@example.com",   // Email
    username: "johndoe",         // Username
    avatar: "https://example.com/avatar.jpg" // Avatar URL
)

// Generirajte SSO žeton
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Posredujte ta žeton vašemu frontendu za avtentikacijo
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Preprosto SSO (za razvoj/testiranje)

```swift
import FastCommentsSwift

// Ustvarite preproste SSO uporabniške podatke (ni potrebnega API ključa)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generirajte preprost SSO žeton
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```