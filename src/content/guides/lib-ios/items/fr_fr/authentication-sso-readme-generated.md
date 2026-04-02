FastComments prend en charge trois modes d'authentification :

1. **Anonymous** -- pas de jeton SSO ; les utilisateurs obtiennent des identités basées sur la session
2. **Simple SSO** -- jeton côté client pour démos et tests (non sécurisé)
3. **Secure SSO** -- jeton signé côté serveur pour la production

### Simple SSO

Utile pour des démonstrations et des tests locaux. N'importe qui peut usurper l'identité de n'importe quel utilisateur avec Simple SSO, ne l'utilisez donc pas en production.

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

`SimpleSSOUserData` prend également en charge les champs optionnels :

- `id` -- ID utilisateur (par défaut l'email si non défini)
- `displayName` -- nom d'affichage distinct
- `displayLabel` -- étiquette personnalisée affichée à côté du nom (par ex. "VIP")
- `websiteUrl` -- lien sur le nom de l'utilisateur
- `locale` -- code de locale
- `isProfileActivityPrivate` -- masquer l'activité du profil (true par défaut)

### Secure SSO

En production, votre backend génère un jeton SSO signé en utilisant votre secret API. L'application iOS récupère ce jeton depuis votre serveur et le transmet à la configuration.

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
// Return this token to your iOS app via your API
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
            // Récupérer le jeton depuis votre backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Créer une nouvelle configuration avec le jeton, ou le définir avant le chargement
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` prend en charge des champs supplémentaires :

- `optedInNotifications` -- inscription aux notifications par e-mail
- `displayLabel` -- étiquette personnalisée
- `displayName` -- nom d'affichage
- `websiteUrl` -- URL du site web
- `groupIds` -- appartenances à des groupes
- `isAdmin` -- privilèges d'administrateur
- `isModerator` -- privilèges de modérateur
- `isProfileActivityPrivate` -- confidentialité du profil

---
---