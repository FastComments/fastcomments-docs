FastComments suporta três modos de autenticação:

1. **Anônimo** -- sem token SSO; os usuários recebem identidades baseadas em sessão
2. **Simple SSO** -- token no cliente para demonstrações e testes (não seguro)
3. **Secure SSO** -- token assinado pelo servidor para produção

### Simple SSO

Útil para demonstrações e testes locais. Qualquer pessoa pode se passar por qualquer usuário com Simple SSO, portanto não o utilize em produção.

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

`SimpleSSOUserData` também suporta campos opcionais:

- `id` -- ID do usuário (padrão é o email se não definido)
- `displayName` -- nome de exibição separado
- `displayLabel` -- etiqueta personalizada exibida ao lado do nome (por exemplo "VIP")
- `websiteUrl` -- link no nome do usuário
- `locale` -- código de localidade
- `isProfileActivityPrivate` -- ocultar atividade do perfil (padrão é true)

### Secure SSO

Em produção, seu backend gera um token SSO assinado usando seu segredo de API. O app iOS busca esse token no seu servidor e o passa para a config.

**No seu backend** (usando o SDK Swift do FastComments ou qualquer linguagem):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Retorne este token para seu app iOS via sua API
```

**No seu app iOS:**

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
            // Busque o token no seu backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Crie uma nova config com o token, ou defina-o antes do carregamento
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` suporta campos adicionais:

- `optedInNotifications` -- opt-in para notificações por email
- `displayLabel` -- etiqueta personalizada
- `displayName` -- nome de exibição
- `websiteUrl` -- URL do site
- `groupIds` -- associações a grupos
- `isAdmin` -- privilégios de administrador
- `isModerator` -- privilégios de moderador
- `isProfileActivityPrivate` -- privacidade do perfil

---
---