### Korištenje javnog API-ja

```swift
import FastCommentsSwift

// Kreirajte API klijenta
let publicApi = PublicAPI()

// Dohvatite komentare za stranicu
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

### Korištenje autentificiranog API-ja

```swift
import FastCommentsSwift

// Kreirajte konfiguraciju s API ključem
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Dohvatite komentare koristeći autentificirani API
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

### Korištenje SSO za autentikaciju

#### Sigurno SSO (Preporučeno za produkciju)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Kreirajte sigurne SSO korisničke podatke (samo na strani poslužitelja!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID korisnika
    email: "user@example.com",   // E-pošta
    username: "johndoe",         // Korisničko ime
    avatar: "https://example.com/avatar.jpg" // URL avatara
)

// Generirajte SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Proslijedite ovaj token svom frontend-u radi autentikacije
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Jednostavan SSO (za razvoj/testiranje)

```swift
import FastCommentsSwift

// Kreirajte jednostavne SSO korisničke podatke (nije potreban API ključ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generirajte jednostavni SSO token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```