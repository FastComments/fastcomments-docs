### Usando la API pública

```swift
import FastCommentsSwift

// Obtener comentarios para una página
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

### Usando la API autenticada

```swift
import FastCommentsSwift

// Configura tu clave API en la configuración compartida (se envía como encabezado x-api-key)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// Obtener comentarios usando la API autenticada
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

### Usando la API de moderación

```swift
import FastCommentsSwift

// Los métodos de moderación están autorizados con un token `sso` para el moderador actuante
// (genéralo con FastCommentsSSO, consulta la sección SSO arriba).
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

### Usando SSO para autenticación

#### SSO seguro (Recomendado para producción)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Crear datos de usuario SSO seguros (solo del lado del servidor)
let userData = SecureSSOUserData(
    id: "user-123",              // ID de usuario
    email: "user@example.com",   // Correo electrónico
    username: "johndoe",         // Nombre de usuario
    avatar: "https://example.com/avatar.jpg" // URL del avatar
)

// Generar token SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // Pasar este token a tu frontend para autenticación
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO simple (Para desarrollo/pruebas)

```swift
import FastCommentsSwift

// Crear datos de usuario SSO simples (no se necesita clave API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// Generar token SSO simple
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```