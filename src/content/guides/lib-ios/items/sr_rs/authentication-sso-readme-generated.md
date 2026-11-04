FastComments подржава три режима аутентификације:

1. **Анонимни** -- без SSO токена; корисници добијају идентитете засноване на сесији
2. **Simple SSO** -- токен на страни клијента за демонстрације и тестирање (није безбедно)
3. **Secure SSO** -- токен потписан на серверу за продукцију

### Simple SSO

Корисно за демонстрације и локално тестирање. Свако може да се представи као било који корисник са Simple SSO, тако да га не користите у продукцији.

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

`SimpleSSOUserData` такође подржава опционе поља:

- `id` -- ID корисника (подразумевано email ако није постављено)
- `displayName` -- засебно име за приказ
- `displayLabel` -- прилагођена ознака приказана поред имена (нпр. "VIP")
- `websiteUrl` -- линк на имену корисника
- `locale` -- код локалитета
- `isProfileActivityPrivate` -- сакрива активност профила (подразумевано true)

### Secure SSO

У продукцији, ваш backend генерише потписани SSO токен користећи вашу API тајну. iOS апликација преузима овај токен са вашег сервера и прослеђује га у конфигурацију.

**На вашем backend-у** (користећи FastComments Swift SDK или било који језик):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Вратите овај токен својој iOS апликацији преко вашег API-ја
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
            // Преузмите токен са вашег backend-а
            let token = try? await fetchSSOTokenFromYourBackend()
            // Креирајте нову конфигурацију са токеном или га поставите пре учитавања
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` подржава додатна поља:

- `optedInNotifications` -- пријава за email обавештења
- `displayLabel` -- прилагођена ознака
- `displayName` -- име за приказ
- `websiteUrl` -- URL веб-сајта
- `groupIds` -- чланства у групама
- `isAdmin` -- администраторске привилегије
- `isModerator` -- модераторске привилегије
- `isProfileActivityPrivate` -- приватност профила

---
---