FastComments поддържа три режима на удостоверяване:

1. **Анонимен** -- няма SSO токен; потребителите получават идентичности, базирани на сесия
2. **Simple SSO** -- клиентски токен за демота и тестове (не е сигурно)
3. **Secure SSO** -- сървърно-подписан токен за продукция

### Simple SSO

Подходящ за демота и локални тестове. Всеки може да се представи като всеки потребител със Simple SSO, затова не го използвайте в продукция.

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

`SimpleSSOUserData` също така поддържа опционални полета:

- `id` -- потребителско ID (по подразбиране е email, ако не е зададено)
- `displayName` -- отделно показвано име
- `displayLabel` -- персонализиран етикет, показван до името (например "VIP")
- `websiteUrl` -- връзка към името на потребителя
- `locale` -- код на локала
- `isProfileActivityPrivate` -- скриване на активността от профила (по подразбиране true)

### Secure SSO

В продукция вашият бекенд генерира подписан SSO токен, използвайки вашия API секрет. iOS приложението извлича този токен от вашия сървър и го подава към конфигурацията.

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
// Върнете този токен на вашето iOS приложение чрез вашето API
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
            // Извлечете токена от вашия бекенд
            let token = try? await fetchSSOTokenFromYourBackend()
            // Създайте нова конфигурация с токена, или го задайте преди зареждане
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` поддържа допълнителни полета:

- `optedInNotifications` -- съгласие за имейл известия
- `displayLabel` -- персонализиран етикет
- `displayName` -- показвано име
- `websiteUrl` -- URL на уебсайт
- `groupIds` -- членства в групи
- `isAdmin` -- администраторски привилегии
- `isModerator` -- модераторски привилегии
- `isProfileActivityPrivate` -- поверителност на профилната активност

---
---