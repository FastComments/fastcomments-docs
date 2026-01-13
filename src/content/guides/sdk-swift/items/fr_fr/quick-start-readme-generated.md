### Utiliser l'API publique

```swift
import FastCommentsSwift

// Crée le client API
let publicApi = PublicAPI()

// Récupère les commentaires pour une page
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

### Utiliser l'API authentifiée

```swift
import FastCommentsSwift

// Crée la configuration avec la clé API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Récupère les commentaires en utilisant l'API authentifiée
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

### Utiliser SSO pour l'authentification

#### SSO sécurisé (recommandé pour la production)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Crée les données utilisateur SSO sécurisées (côté serveur uniquement !)
let userData = SecureSSOUserData(
    id: "user-123",              // ID utilisateur
    email: "user@example.com",   // Adresse e-mail
    username: "johndoe",         // Nom d'utilisateur
    avatar: "https://example.com/avatar.jpg" // URL de l'avatar
)

// Génère le jeton SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Transmettez ce jeton à votre frontend pour l'authentification
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO simple (pour le développement/tests)

```swift
import FastCommentsSwift

// Crée les données utilisateur SSO simples (aucune clé API nécessaire)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Génère le jeton SSO simple
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```