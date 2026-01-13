### Usando a API Pública

```swift
import FastCommentsSwift

// Criar cliente da API
let publicApi = PublicAPI()

// Buscar comentários de uma página
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

### Usando a API Autenticada

```swift
import FastCommentsSwift

// Criar configuração com chave de API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Buscar comentários usando API autenticada
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

### Usando SSO para Autenticação

#### SSO Seguro (Recomendado para Produção)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Criar dados de usuário SSO seguro (somente no servidor!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID do usuário
    email: "user@example.com",   // E-mail
    username: "johndoe",         // Nome de usuário
    avatar: "https://example.com/avatar.jpg" // URL do avatar
)

// Gerar token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Passe este token para seu frontend para autenticação
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO Simples (Para Desenvolvimento/Testes)

```swift
import FastCommentsSwift

// Criar dados de usuário SSO simples (não é necessária chave de API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Gerar token SSO simples
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```