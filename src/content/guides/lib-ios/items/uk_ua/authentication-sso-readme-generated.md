FastComments підтримує три режими автентифікації:

1. **Анонімний** -- без SSO-токена; користувачі отримують ідентичності на основі сесії
2. **Простий SSO** -- клієнтський токен для демо та тестування (небезпечний)
3. **Захищений SSO** -- токен, підписаний сервером, для продакшену

### Простий SSO

Корисний для демо та локального тестування. Будь-хто може видавати себе за будь-якого користувача за допомогою Simple SSO, тому не використовуйте його в продакшені.

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

`SimpleSSOUserData` також підтримує необов'язкові поля:

- `id` -- ідентифікатор користувача (за замовчуванням email, якщо не задано)
- `displayName` -- окреме ім'я для відображення
- `displayLabel` -- власна мітка, що показується поруч з ім'ям (наприклад, "VIP")
- `websiteUrl` -- посилання на імені користувача
- `locale` -- код локалі
- `isProfileActivityPrivate` -- приховати активність профілю (за замовчуванням true)

### Захищений SSO

У продакшені ваш бекенд генерує підписаний SSO-токен, використовуючи ваш API-секрет. iOS-застосунок отримує цей токен з вашого сервера та передає його в конфігурацію.

**На вашому бекенді** (використовуючи FastComments Swift SDK або будь-яку мову):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Поверніть цей токен вашому iOS-застосунку через ваш API
```

**У вашому iOS-застосунку:**

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
            // Отримайте токен з вашого бекенду
            let token = try? await fetchSSOTokenFromYourBackend()
            // Створіть нову конфігурацію з токеном або задайте його перед завантаженням
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` підтримує додаткові поля:

- `optedInNotifications` -- згода на email-сповіщення
- `displayLabel` -- власна мітка
- `displayName` -- ім'я для відображення
- `websiteUrl` -- URL вебсайту
- `groupIds` -- членство в групах
- `isAdmin` -- права адміністратора
- `isModerator` -- права модератора
- `isProfileActivityPrivate` -- приватність профілю

---
---