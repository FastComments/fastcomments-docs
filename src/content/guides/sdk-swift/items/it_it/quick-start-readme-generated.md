### Utilizzo dell'API pubblica

```swift
import FastCommentsSwift

// Crea il client API
let publicApi = PublicAPI()

// Recupera i commenti per una pagina
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

### Utilizzo dell'API autenticata

```swift
import FastCommentsSwift

// Crea la configurazione con la chiave API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Recupera commenti usando l'API autenticata
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

### Utilizzo dell'API di moderazione

```swift
import FastCommentsSwift

// I metodi di moderazione sono autorizzati con un token `sso` per il moderatore attivo
// (generalo con FastCommentsSSO, vedi la sezione SSO sopra).
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

### Utilizzo di SSO per l'autenticazione

#### SSO sicuro (consigliato per la produzione)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Crea i dati utente SSO sicuri (solo lato server!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID utente
    email: "user@example.com",   // Email
    username: "johndoe",         // Nome utente
    avatar: "https://example.com/avatar.jpg" // URL dell'avatar
)

// Genera il token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Passa questo token al tuo frontend per l'autenticazione
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO semplice (per sviluppo/test)

```swift
import FastCommentsSwift

// Crea i dati utente SSO semplici (non è necessaria una chiave API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Genera il token SSO semplice
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```