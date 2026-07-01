### Korištenje javnog API-ja

```swift
import FastCommentsSwift

// Dohvati komentare za stranicu
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

### Korištenje autentificiranog API-ja

```swift
import FastCommentsSwift

// Konfigurirajte svoj API ključ u zajedničkoj konfiguraciji (poslan kao zaglavlje x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Dohvati komentare koristeći autentificirani API
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

### Korištenje API-ja za moderiranje

```swift
import FastCommentsSwift

// Metode moderiranja su autorizirane pomoću `sso` tokena za djelujućeg moderatora
// (generirajte ga pomoću FastCommentsSSO, pogledajte odjeljak SSO iznad).
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

### Korištenje SSO za autentifikaciju

#### Sigurni SSO (Preporučeno za produkciju)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Stvori sigurne SSO korisničke podatke (samo na poslužitelju!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID korisnika
    email: "user@example.com",   // Email
    username: "johndoe",         // Korisničko ime
    avatar: "https://example.com/avatar.jpg" // URL avatara
)

// Generiraj SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Proslijedi ovaj token vašem frontend-u za autentifikaciju
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Jednostavni SSO (Za razvoj/testiranje)

```swift
import FastCommentsSwift

// Stvori jednostavne SSO korisničke podatke (nije potreban API ključ)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generiraj jednostavni SSO token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```