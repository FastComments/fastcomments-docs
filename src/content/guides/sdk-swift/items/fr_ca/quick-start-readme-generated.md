### Utilisation de l’API publique

```swift
import FastCommentsSwift

// Récupérer les commentaires pour une page
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

### Utilisation de l’API authentifiée

```swift
import FastCommentsSwift

// Configurez votre clé API dans la configuration partagée (envoyée en tant qu’en-tête x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Récupérer les commentaires en utilisant l'API authentifiée
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

### Utilisation de l’API de modération

```swift
import FastCommentsSwift

// Les méthodes de modération sont autorisées avec un jeton `sso` pour le modérateur en exercice
// (générez-le avec FastCommentsSSO, voir la section SSO ci‑dessus).
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

### Utilisation de SSO pour l’authentification

#### SSO sécurisé (recommandé pour la production)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Créer des données d’utilisateur SSO sécurisées (serveur uniquement !)
let userData = SecureSSOUserData(
    id: "user-123",              // ID d’utilisateur
    email: "user@example.com",   // Courriel
    username: "johndoe",         // Nom d’utilisateur
    avatar: "https://example.com/avatar.jpg" // URL de l’avatar
)

// Générer le jeton SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Transmettre ce jeton à votre interface frontal pour l’authentification
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO simple (pour le développement/les tests)

```swift
import FastCommentsSwift

// Créer des données d’utilisateur SSO simples (aucune clé API requise)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Générer le jeton SSO simple
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```