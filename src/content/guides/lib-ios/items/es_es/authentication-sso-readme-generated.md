FastComments admite tres modos de autenticación:

1. **Anónimo** -- sin token SSO; los usuarios obtienen identidades basadas en sesión
2. **SSO Simple** -- token del lado del cliente para demos y pruebas (no es seguro)
3. **SSO Seguro** -- token firmado por el servidor para producción

### SSO Simple

Útil para demos y pruebas locales. Cualquiera puede suplantar a cualquier usuario con SSO Simple, así que no lo uses en producción.

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

`SimpleSSOUserData` también admite campos opcionales:

- `id` -- ID del usuario (por defecto es el email si no se establece)
- `displayName` -- nombre para mostrar separado
- `displayLabel` -- etiqueta personalizada mostrada junto al nombre (por ejemplo, "VIP")
- `websiteUrl` -- enlace en el nombre del usuario
- `locale` -- código de configuración regional
- `isProfileActivityPrivate` -- ocultar la actividad del perfil (por defecto es true)

### SSO Seguro

En producción, tu backend genera un token SSO firmado usando tu secreto de API. La app de iOS obtiene este token de tu servidor y lo pasa a la configuración.

**En tu backend** (usando el SDK de Swift de FastComments o cualquier lenguaje):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Devuelve este token a tu app de iOS a través de tu API
```

**En tu app de iOS:**

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
                ProgressView("Cargando...")
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

`SecureSSOUserData` admite campos adicionales:

- `optedInNotifications` -- aceptación de notificaciones por email
- `displayLabel` -- etiqueta personalizada
- `displayName` -- nombre para mostrar
- `websiteUrl` -- URL del sitio web
- `groupIds` -- membresías de grupos
- `isAdmin` -- privilegios de administrador
- `isModerator` -- privilegios de moderador
- `isProfileActivityPrivate` -- privacidad del perfil

---
---