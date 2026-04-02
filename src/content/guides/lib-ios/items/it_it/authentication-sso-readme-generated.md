FastComments supporta tre modalità di autenticazione:

1. **Anonimo** -- nessun token SSO; gli utenti ottengono identità basate sulla sessione
2. **SSO semplice** -- token lato client per demo e test (non sicuro)
3. **SSO sicuro** -- token firmato dal server per produzione

### SSO semplice

Utile per demo e test locali. Chiunque può impersonare qualsiasi utente con SSO semplice, quindi non usarlo in produzione.

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

`SimpleSSOUserData` supporta anche campi opzionali:

- `id` -- ID utente (predefinito all'email se non impostato)
- `displayName` -- nome visualizzato separato
- `displayLabel` -- etichetta personalizzata mostrata accanto al nome (es. "VIP")
- `websiteUrl` -- link sul nome dell'utente
- `locale` -- codice locale
- `isProfileActivityPrivate` -- nascondi attività del profilo (predefinito: true)

### SSO sicuro

In produzione, il tuo backend genera un token SSO firmato usando il tuo API secret. L'app iOS recupera questo token dal tuo server e lo passa alla config.

**Sul tuo backend** (usando il FastComments Swift SDK o qualsiasi linguaggio):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Restituisci questo token alla tua app iOS tramite la tua API
```

**Nella tua app iOS:**

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
            // Recupera il token dal tuo backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Crea una nuova config con il token, o impostalo prima del caricamento
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` supporta campi aggiuntivi:

- `optedInNotifications` -- opt-in alle notifiche email
- `displayLabel` -- etichetta personalizzata
- `displayName` -- nome visualizzato
- `websiteUrl` -- URL del sito web
- `groupIds` -- appartenenze ai gruppi
- `isAdmin` -- privilegi di admin
- `isModerator` -- privilegi di moderatore
- `isProfileActivityPrivate` -- privacy del profilo

---
---