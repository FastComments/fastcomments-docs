FastComments prend en charge trois modes d'authentification :

1. **Anonymous** -- pas de SSO token; les utilisateurs obtiennent des identités basées sur la session
2. **Simple SSO** -- token côté client pour les démonstrations et les tests (non sécurisé)
3. **Secure SSO** -- token signé côté serveur pour la production

### Simple SSO

Utile pour les démonstrations et les tests locaux. N'importe qui peut usurper l'identité de n'importe quel utilisateur avec Simple SSO, donc ne l'utilisez pas en production.

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

- `id` -- ID utilisateur (par défaut l'email si non défini)
- `displayName` -- nom affiché distinct
- `displayLabel` -- étiquette personnalisée affichée à côté du nom (p. ex. "VIP")
- `websiteUrl` -- lien sur le nom de l'utilisateur
- `locale` -- code de la locale
- `isProfileActivityPrivate` -- masquer l'activité du profil (par défaut true)

### Secure SSO

En production, votre backend génère un token SSO signé en utilisant votre secret d'API. L'application iOS récupère ce token depuis votre serveur et le transmet à la configuration.

**Sur votre backend** (en utilisant le FastComments Swift SDK ou n'importe quel langage):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Retournez ce jeton à votre application iOS via votre API
```

**Dans votre application iOS :**

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
            // Récupérer le jeton depuis votre backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Créez une nouvelle configuration avec le jeton, ou définissez-le avant le chargement
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` prend en charge des champs supplémentaires :

- `optedInNotifications` -- inscription aux notifications par courriel
- `displayLabel` -- étiquette personnalisée
- `displayName` -- nom affiché
- `websiteUrl` -- URL du site web
- `groupIds` -- appartenances à des groupes
- `isAdmin` -- privilèges d'administrateur
- `isModerator` -- privilèges de modérateur
- `isProfileActivityPrivate` -- confidentialité du profil

---
---