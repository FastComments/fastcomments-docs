FastComments подржава три режима аутентификације:

1. **Anonymous** -- нема SSO токена; корисници добијају идентитете засноване на сесији
2. **Simple SSO** -- клијентски токен за демонстрације и тестирање (није безбедан)
3. **Secure SSO** -- серверски потписан токен за продукцију

### Simple SSO

Корисно за демонстрације и локално тестирање. Са Simple SSO свако може да се представи као било који корисник, па га не користите у продукцији.

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

- `id` -- ID корисника (подразумева се на адресу е-поште ако није подешено)
- `displayName` -- посебно име за приказ
- `displayLabel` -- прилагођена ознака приказана поред имена (нпр. "VIP")
- `websiteUrl` -- веза на корисниково име
- `locale` -- код локала
- `isProfileActivityPrivate` -- сакриј активност профила (подразумевано true)

### Secure SSO

У продукцији, ваш бекенд генерише потписан SSO токен користећи ваш API секрет. iOS апликација преузима овај токен са вашег сервера и прослеђује га у конфигурацију.

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
// Вратите овај токен вашој iOS апликацији преко вашег API-ја
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
            // Преузмите токен са вашег бекенда
            let token = try? await fetchSSOTokenFromYourBackend()
            // Креирајте нову конфигурацију са токеном, или је подесите пре учитавања
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` подржава додатна поља:

- `optedInNotifications` -- пријава за обавештења путем е-поште
- `displayLabel` -- прилагођена ознака
- `displayName` -- име за приказ
- `websiteUrl` -- URL веб-сајта
- `groupIds` -- чланства у групама
- `isAdmin` -- администраторска права
- `isModerator` -- права модератора
- `isProfileActivityPrivate` -- приватност активности профила