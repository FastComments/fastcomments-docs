FastComments admite tres modos de autenticación:

1. **Anonymous** -- sin token SSO; los usuarios obtienen identidades basadas en sesión
2. **Simple SSO** -- token del lado del cliente para demos y pruebas (no seguro)
3. **Secure SSO** -- token firmado por el servidor para producción

### Simple SSO

Útil para demos y pruebas locales. Cualquiera puede suplantar a cualquier usuario con Simple SSO, por lo que no lo uses en producción.

```swift
import FastCommentsSwift

let userData = SimpleSSOUserData(
    username: "Jane Doe",
    email: "jane@example.com",
    avatar: "https://example.com/avatar.jpg"
)
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
let token = try? sso.prepareToSend()

let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page-1",
    sso: token
)
let sdk = FastCommentsSDK(config: config)
```

`SimpleSSOUserData` also supports optional fields:

- `id` -- ID de usuario (por defecto email si no se establece)
- `displayName` -- nombre para mostrar separado
- `displayLabel` -- etiqueta personalizada mostrada junto al nombre (por ejemplo "VIP")
- `websiteUrl` -- enlace en el nombre del usuario
- `locale` -- código de localización
- `isProfileActivityPrivate` -- ocultar la actividad del perfil (por defecto true)

### Secure SSO

En producción, tu backend genera un token SSO firmado usando tu secreto de API. La app iOS obtiene este token desde tu servidor y lo pasa a la configuración.

**On your backend** (using the FastComments Swift SDK or any language):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Devuelve este token a tu app iOS mediante tu API
```

**In your iOS app:**

```swift
struct MyView: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-page-1"
        )
    )
    @State private var isLoadingToken = true

    var body: some View {
        Group {
            if isLoadingToken {
                ProgressView("Loading...")
            } else {
                FastCommentsView(sdk: sdk)
            }
        }
        .task {
            // Obtén el token de tu backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Crea una nueva configuración con el token, o establécelo antes de cargar
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` supports additional fields:

- `optedInNotifications` -- suscripción a notificaciones por correo electrónico
- `displayLabel` -- etiqueta personalizada
- `displayName` -- nombre para mostrar
- `websiteUrl` -- URL del sitio web
- `groupIds` -- pertenencia a grupos
- `isAdmin` -- privilegios de administrador
- `isModerator` -- privilegios de moderador
- `isProfileActivityPrivate` -- privacidad del perfil

---
---