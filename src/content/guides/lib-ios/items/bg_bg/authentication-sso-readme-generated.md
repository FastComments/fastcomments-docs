FastComments поддържа три режима на автентикация:

1. **Анонимен** -- без SSO токен; потребителите получават идентичности, базирани на сесия
2. **Simple SSO** -- токен от клиентската страна за демо и тестване (не е защитен)
3. **Secure SSO** -- токен, подписан от сървъра, за продукционна среда

### Simple SSO

Полезен за демо и локално тестване. Всеки може да се представи за всеки потребител чрез Simple SSO, затова не го използвайте в продукционна среда.

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

`SimpleSSOUserData` поддържа и опционални полета:

- `id` -- потребителски ID (по подразбиране е имейлът, ако не е зададен)
- `displayName` -- отделно име за показване
- `displayLabel` -- персонализиран етикет, показван до името (напр. "VIP")
- `websiteUrl` -- линк върху името на потребителя
- `locale` -- код на локала
- `isProfileActivityPrivate` -- скриване на активността в профила (по подразбиране е true)

### Secure SSO

В продукционна среда вашият backend генерира подписан SSO токен с вашия API secret. iOS приложението извлича този токен от вашия сървър и го предава в конфигурацията.

**На вашия backend** (използвайки FastComments Swift SDK или друг език):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Върнете този токен към вашето iOS приложение чрез вашия API
```

**Във вашето iOS приложение:**

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
            // Извлечете токена от вашия backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Създайте нова конфигурация с токена или го задайте преди зареждане
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` поддържа допълнителни полета:

- `optedInNotifications` — съгласие за имейл известия
- `displayLabel` — персонализиран етикет
- `displayName` — име за показване
- `websiteUrl` — URL на уебсайт
- `groupIds` — членства в групи
- `isAdmin` — администраторски права
- `isModerator` — модераторски права
- `isProfileActivityPrivate` — поверителност на профила

---