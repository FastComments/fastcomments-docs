---
FastComments подржава три начина аутентификације:

1. **Anonymous** -- нема SSO токена; корисници добијају идентитете засноване на сесији
2. **Simple SSO** -- токен на клијентској страни за демонстрације и тестирање (није сигурно)
3. **Secure SSO** -- серверски потписан токен за продукцију

### Simple SSO

Погодно за демонстрације и локално тестирање. Са Simple SSO свако може преузети идентитет било ког корисника, па га не користите у продукцији.

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

`SimpleSSOUserData` такође подржава опционална поља:

- `id` -- ID корисника (ако није постављено, подразумева се email)
- `displayName` -- засебно приказано име
- `displayLabel` -- прилагођени натпис приказан поред имена (нпр. "VIP")
- `websiteUrl` -- веза на корисниковом имену
- `locale` -- код локализације
- `isProfileActivityPrivate` -- сакриј активност профила (задано: true)

### Secure SSO

У продукцији, ваш backend генерише потписани SSO token користећи ваш API secret. iOS апликација преузима тај token са вашег сервера и просљеђује га у конфигурацију.

**На вашем backendu** (користећи FastComments Swift SDK или било који језик):

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

**У вашој iOS апликацији:**

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
            // Fetch the token from your backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Create a new config with the token, or set it before load
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` подржава додатна поља:

- `optedInNotifications` -- пријава за e-mail обавјештења
- `displayLabel` -- прилагођени натпис
- `displayName` -- приказано име
- `websiteUrl` -- веб адреса
- `groupIds` -- чланства у групама
- `isAdmin` -- администраторске привилегије
- `isModerator` -- модераторске привилегије
- `isProfileActivityPrivate` -- приватност активности профила

---
---