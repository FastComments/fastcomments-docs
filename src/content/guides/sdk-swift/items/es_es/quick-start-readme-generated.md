### Uso de la API Pública

```swift
import FastCommentsSwift

// Crear cliente de la API
let publicApi = PublicAPI()

// Obtener comentarios para una página
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

### Uso de la API Autenticada

```swift
import FastCommentsSwift

// Crear configuración con la clave API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// Obtener comentarios usando la API autenticada
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

### Uso de SSO para la autenticación

#### SSO seguro (recomendado para producción)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// Crear datos de usuario SSO seguros (¡solo del lado del servidor!)
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
    // Pase este token a su frontend para la autenticación
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO simple (para desarrollo/pruebas)

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