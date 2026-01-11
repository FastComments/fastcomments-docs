### Korišćenje javnog API-ja

```swift
import FastCommentsSwift

// Kreiraj API klijenta
let publicApi = PublicAPI()

// Preuzmi komentare za stranicu
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

### Korišćenje autentifikovanog API-ja

```swift
import FastCommentsSwift

// Kreiraj konfiguraciju sa API ključem
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Preuzmi komentare koristeći autentifikovani API
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

### Korišćenje SSO za autentifikaciju

#### Siguran SSO (Preporučeno za produkciju)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Kreiraj sigurne SSO podatke o korisniku (samo na serverskoj strani!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID korisnika
    email: "user@example.com",   // Email
    username: "johndoe",         // Korisničko ime
    avatar: "https://example.com/avatar.jpg" // URL avatara
)

// Generiši SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Prosledi ovaj token svom frontend-u za autentifikaciju
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Jednostavan SSO (Za razvoj/testiranje)

```swift
import FastCommentsSwift

// Kreiraj jednostavne SSO podatke o korisniku (nije potreban API ključ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generiši jednostavan SSO token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```