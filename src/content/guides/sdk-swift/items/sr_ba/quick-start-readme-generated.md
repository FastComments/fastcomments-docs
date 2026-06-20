### Korištenje javnog API-ja

```swift
import FastCommentsSwift

// Kreirajte API klijent
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

### Korištenje autentifikovanog API-ja

```swift
import FastCommentsSwift

// Kreirajte konfiguraciju sa API ključem
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Dohvatite komentare koristeći autentifikovani API
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

### Korištenje API-ja za moderaciju

```swift
import FastCommentsSwift

// Metode moderacije se ovlašćuju `sso` tokenom za moderatora koji djeluje
// (generišite ga pomoću FastCommentsSSO, pogledajte SSO odjeljak iznad).
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

### Korištenje SSO za autentifikaciju

#### Sigurni SSO (Preporučeno za produkciju)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Kreirajte sigurne SSO podatke korisnika (samo na serverskoj strani!)
let userData = SecureSSOUserData(
    id: "user-123",              // User ID
    email: "user@example.com",   // Email
    username: "johndoe",         // Username
    avatar: "https://example.com/avatar.jpg" // Avatar URL
)

// Generišite SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Proslijedite ovaj token svom frontend-u za autentifikaciju
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Jednostavni SSO (Za razvoj/testiranje)

```swift
import FastCommentsSwift

// Kreirajte jednostavne SSO podatke korisnika (nije potreban API ključ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generišite jednostavni SSO token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```