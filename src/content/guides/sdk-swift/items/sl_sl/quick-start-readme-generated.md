### Uporaba javnega API-ja

```swift
import FastCommentsSwift

// Ustvari API odjemalca
let publicApi = PublicAPI()

// Pridobi komentarje za stran
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

### Uporaba avtenticiranega API-ja

```swift
import FastCommentsSwift

// Ustvari konfiguracijo z API ključem
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Pridobi komentarje z avtenticiranim API-jem
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

### Uporaba SSO za avtentikacijo

#### Varen SSO (priporočeno za produkcijo)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Ustvari varne SSO podatke o uporabniku (samo na strežniški strani!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID uporabnika
    email: "user@example.com",   // E-pošta
    username: "johndoe",         // Uporniško ime
    avatar: "https://example.com/avatar.jpg" // URL avatarja
)

// Generiraj SSO žeton
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Posredujte ta žeton na vaš frontend za avtentikacijo
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Enostaven SSO (za razvoj/testiranje)

```swift
import FastCommentsSwift

// Ustvari enostavne SSO podatke o uporabniku (API ključ ni potreben)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generiraj enostaven SSO žeton
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```