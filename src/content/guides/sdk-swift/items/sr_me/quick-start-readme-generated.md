### Korišćenje javnog API-ja

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

### Korišćenje autentifikovanog API-ja

```swift
import FastCommentsSwift

// Konfigurišite svoj API ključ u zajedničkoj konfiguraciji (šalje se kao x-api-key zaglavlje)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Dohvati komentare koristeći autentifikovani API
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

### Korišćenje API-ja za moderaciju

```swift
import FastCommentsSwift

// Metode moderacije su autorizovane pomoću `sso` tokena za djelujućeg moderatora
// (generišite ga pomoću FastCommentsSSO, pogledajte odeljak SSO iznad).
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

### Korišćenje SSO za autentifikaciju

#### Siguran SSO (Preporučeno za produkciju)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Kreiraj sigurnu SSO korisničku podatke (samo na serveru!)
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
    // Proslijedi ovaj token svom frontendu za autentifikaciju
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### Jednostavan SSO (Za razvoj/testiranje)

```swift
import FastCommentsSwift

// Kreiraj jednostavne SSO korisničke podatke (API ključ nije potreban)
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