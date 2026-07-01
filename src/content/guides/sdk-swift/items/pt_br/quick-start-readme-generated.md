### Usando a API Pública

```swift
import FastCommentsSwift

// Buscar comentários para uma página
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

### Usando a API Autenticada

```swift
import FastCommentsSwift

// Configure sua chave de API na configuração compartilhada (enviada como o cabeçalho x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Buscar comentários usando a API autenticada
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

### Usando a API de Moderação

```swift
import FastCommentsSwift

// Métodos de moderação são autorizados com um token `sso` para o moderador atuante
// (gere-o com FastCommentsSSO, veja a seção SSO acima).
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

### Usando SSO para Autenticação

#### SSO Seguro (Recomendado para Produção)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Criar dados de usuário SSO seguros (apenas no lado do servidor!)
let userData = SecureSSOUserData(
    id: "user-123",              // ID do Usuário
    email: "user@example.com",   // Email
    username: "johndoe",         // Nome de usuário
    avatar: "https://example.com/avatar.jpg" // URL do avatar
)

// Gerar token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Passe este token ao seu front-end para autenticação
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