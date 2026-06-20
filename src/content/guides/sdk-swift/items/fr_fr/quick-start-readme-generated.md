---
### Utilisation de l'API publique

```swift
import FastCommentsSwift

// Créer le client API
let publicApi = PublicAPI()

// Fetch comments for a page
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

### Utilisation de l'API authentifiée

```swift
import FastCommentsSwift

// Créer la configuration avec la clé API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Fetch comments using authenticated API
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

### Utilisation de l'API de modération

```swift
import FastCommentsSwift

// Les méthodes de modération sont autorisées avec un jeton `sso` pour le modérateur agissant
// (générez-le avec FastCommentsSSO, voir la section SSO ci-dessus).
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

### Utilisation du SSO pour l'authentification

#### SSO sécurisé (Recommandé pour la production)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Créer des données utilisateur SSO sécurisées (uniquement côté serveur !)
let userData = SecureSSOUserData(
    id: "user-123",              // ID de l'utilisateur
    email: "user@example.com",   // Email
    username: "johndoe",         // Nom d'utilisateur
    avatar: "https://example.com/avatar.jpg" // URL de l'avatar
)

// Generate SSO token
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Pass this token to your frontend for authentication
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO simple (Pour le développement/test)

```swift
import FastCommentsSwift

// Create simple SSO user data (no API key needed)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generate simple SSO token
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```
---