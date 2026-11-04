O FastComments suporta três modos de autenticação:

1. **Anônimo** -- sem token SSO; os usuários recebem identidades baseadas em sessão
2. **SSO Simples** -- token do lado do cliente para demonstrações e testes (não seguro)
3. **SSO Seguro** -- token assinado pelo servidor para produção

### SSO Simples

Útil para demonstrações e testes locais. Qualquer pessoa pode se passar por qualquer usuário com o SSO Simples, portanto não o use em produção.

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

- `id` -- ID do usuário (usa o e-mail como padrão se não for definido)
- `displayName` -- nome de exibição separado
- `displayLabel` -- rótulo personalizado exibido ao lado do nome (ex.: "VIP")
- `websiteUrl` -- link no nome do usuário
- `locale` -- código de localidade
- `isProfileActivityPrivate` -- ocultar atividade do perfil (padrão: true)

### SSO Seguro

Em produção, seu backend gera um token SSO assinado usando seu segredo de API. O aplicativo iOS busca esse token do seu servidor e o passa para a configuração.

**No seu backend** (usando o FastComments Swift SDK ou qualquer linguagem):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Retorne este token ao seu aplicativo iOS via sua API
```

**No seu aplicativo iOS:**

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
                ProgressView("Carregando...")
            } else {
                FastCommentsView(sdk: sdk)
            }
        }
        .task {
            // Busque o token do seu backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Crie uma nova configuração com o token, ou defina-o antes de carregar
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` suporta campos adicionais:

- `optedInNotifications` -- aceitação de notificações por e-mail
- `displayLabel` -- rótulo personalizado
- `displayName` -- nome de exibição
- `websiteUrl` -- URL do site
- `groupIds` -- associações a grupos
- `isAdmin` -- privilégios de administrador
- `isModerator` -- privilégios de moderador
- `isProfileActivityPrivate` -- privacidade do perfil

---
---