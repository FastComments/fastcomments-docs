FastComments підтримує три режими автентифікації:

1. **Анонімний** -- без SSO-токена; користувачі отримують ідентичності, засновані на сесії
2. **Простий SSO** -- токен на боці клієнта для демонстрацій і тестування (небезпечно)
3. **Захищений SSO** -- серверно-підписаний токен для продакшену

### Простий SSO

Корисно для демонстрацій і локального тестування. Будь-хто може видати себе за будь-якого користувача за допомогою Простого SSO, тому не використовуйте його в продакшені.

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

- `id` -- ідентифікатор користувача (по замовчуванню використовується email, якщо не вказано)
- `displayName` -- окреме ім'я для відображення
- `displayLabel` -- користувацька мітка, показана поруч з ім'ям (наприклад, "VIP")
- `websiteUrl` -- посилання на імені користувача
- `locale` -- код локалі
- `isProfileActivityPrivate` -- приховати активність профілю (за замовчуванням true)

### Захищений SSO

У продакшені ваш сервер генерує підписаний SSO-токен за допомогою вашого API секрета. iOS-додаток отримує цей токен з вашого сервера і передає його в конфігурацію.

**На вашому бэкенді** (з використанням FastComments Swift SDK або на будь-якій мові):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Верніть цей токен у ваш iOS-додаток через ваш API
```

**У вашому iOS-додатку:**

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
            // Отримайте токен з вашого бэкенду
            let token = try? await fetchSSOTokenFromYourBackend()
            // Створіть нову конфігурацію з токеном або встановіть його перед завантаженням
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` підтримує додаткові поля:

- `optedInNotifications` -- підписка на email-сповіщення
- `displayLabel` -- користувацька мітка
- `displayName` -- ім'я для відображення
- `websiteUrl` -- URL веб-сайту
- `groupIds` -- належність до груп
- `isAdmin` -- права адміністратора
- `isModerator` -- права модератора
- `isProfileActivityPrivate` -- приватність активності профілю

---
---