### Utilizzo dell'API Pubblica

```swift
import FastCommentsSwift

// Recupera i commenti per una pagina
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

### Utilizzo dell'API Autenticata

```swift
import FastCommentsSwift

// Configura la tua chiave API nella configurazione condivisa (inviata come intestazione x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Recupera i commenti usando l'API autenticata
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

### Utilizzo dell'API di Moderazione

```swift
import FastCommentsSwift

// I metodi di moderazione sono autorizzati con un token `sso` per il moderatore attivo
// (generalo con FastCommentsSSO, vedi la sezione SSO sopra).
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

### Utilizzo di SSO per l'Autenticazione

#### SSO Sicuro (Consigliato per la Produzione)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Crea dati utente SSO sicuri (solo lato server!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID Utente
    email: "user@example.com",   // Email
    username: "johndoe",         // Nome utente
    avatar: "https://example.com/avatar.jpg" // URL Avatar
)

// Genera token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Passa questo token al tuo frontend per l'autenticazione
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO Semplice (Per Sviluppo/Test)

```swift
import FastCommentsSwift

// Crea dati utente SSO semplici (non è necessaria una chiave API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Genera token SSO semplice
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```