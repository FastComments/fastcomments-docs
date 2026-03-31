FastComments obsługuje trzy tryby uwierzytelniania:

1. **Anonimowy** -- brak tokena SSO; użytkownicy otrzymują tożsamości oparte na sesji
2. **Proste SSO** -- token po stronie klienta do demo i testów (niebezpieczne)
3. **Bezpieczne SSO** -- token podpisany przez serwer do produkcji

### Proste SSO

Przydatne do demo i testów lokalnych. Każdy może podszyć się pod dowolnego użytkownika przy użyciu Prostego SSO, więc nie używaj go w produkcji.

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

`SimpleSSOUserData` obsługuje również opcjonalne pola:

- `id` -- identyfikator użytkownika (domyślnie `email`, jeśli nie ustawiono)
- `displayName` -- osobna nazwa wyświetlana
- `displayLabel` -- niestandardowa etykieta pokazywana obok nazwy (np. "VIP")
- `websiteUrl` -- odnośnik przy nazwie użytkownika
- `locale` -- kod lokalizacji
- `isProfileActivityPrivate` -- ukryj aktywność profilu (domyślnie `true`)

### Bezpieczne SSO

W produkcji twój backend generuje podpisany token SSO używając sekretu API. Aplikacja iOS pobiera ten token z twojego serwera i przekazuje go do konfiguracji.

**Po stronie backendu** (używając FastComments Swift SDK lub dowolnego języka):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Zwróć ten token do swojej aplikacji iOS za pośrednictwem API
```

**W Twojej aplikacji iOS:**

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
            // Pobierz token z twojego backendu
            let token = try? await fetchSSOTokenFromYourBackend()
            // Utwórz nową konfigurację z tokenem lub ustaw go przed załadowaniem
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` obsługuje dodatkowe pola:

- `optedInNotifications` -- zgoda na powiadomienia e-mail
- `displayLabel` -- niestandardowa etykieta
- `displayName` -- nazwa wyświetlana
- `websiteUrl` -- URL strony internetowej
- `groupIds` -- identyfikatory grup
- `isAdmin` -- uprawnienia administratora
- `isModerator` -- uprawnienia moderatora
- `isProfileActivityPrivate` -- ustawienie prywatności aktywności profilu

---
---