FastComments подржава три режима аутентификације:

1. **Анонимно** -- без SSO токена; корисници добијају идентитете засноване на сесији
2. **Једноставан SSO** -- клијентски токен за демое и тестирање (није безбедно)
3. **Безбедан SSO** -- серверски потписан токен за продукцију

### Једноставан SSO

Корисно за демое и локално тестирање. Било ко може се претварати као било који корисник уз Једноставан SSO, па га не користите у продукцији.

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

- `id` -- ID корисника (подразумевано email ако није подешено)
- `displayName` -- одвојено име за приказ
- `displayLabel` -- прилагођена ознака приказана поред имена (нпр. "VIP")
- `websiteUrl` -- линк на име корисника
- `locale` -- код локала
- `isProfileActivityPrivate` -- сакрити активност профила (подразумевано true)

### Безбедан SSO

У продукцији, ваш бекенд генерише потписани SSO токен користећи ваш API тајни кључ. iOS апликација преузима овај токен са вашег сервера и прослеђује га у конфигурацију.

**На вашем бекенду** (користећи FastComments Swift SDK или било који језик):

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
            // Преузми токен са вашег бекенда
            let token = try? await fetchSSOTokenFromYourBackend()
            // Направи нову конфигурацију са токеном, или је подеси пре учитавања
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` подржава додатна поља:

- `optedInNotifications` -- опција пријаве за примање обавештења путем е-поште
- `displayLabel` -- прилагођена ознака
- `displayName` -- име за приказ
- `websiteUrl` -- URL веб сајта
- `groupIds` -- чланства у групама
- `isAdmin` -- админ привилегије
- `isModerator` -- привилегије модератора
- `isProfileActivityPrivate` -- приватност активности профила

---
---