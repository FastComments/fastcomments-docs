---
### Utilisation de l'API publique

```swift
import FastCommentsSwift

// Créer le client API
let publicApi = PublicAPI()

// Récupérer les commentaires d'une page
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

// Récupérer les commentaires en utilisant l'API authentifiée
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

### Utilisation du SSO pour l'authentification

#### SSO sécurisé (Recommandé pour la production)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Créer les données utilisateur SSO sécurisées (côté serveur seulement !)
let userData = SecureSSOUserData(
    id: "user-123",              // ID de l'utilisateur
    email: "user@example.com",   // Courriel
    username: "johndoe",         // Nom d'utilisateur
    avatar: "https://example.com/avatar.jpg" // URL de l'avatar
)

// Générer le jeton SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Transmettez ce jeton à votre frontend pour l'authentification
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO simple (Pour le développement/les tests)

```swift
import FastCommentsSwift

// Créer les données utilisateur SSO simples (aucune clé API requise)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Générer un jeton SSO simple
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```
---