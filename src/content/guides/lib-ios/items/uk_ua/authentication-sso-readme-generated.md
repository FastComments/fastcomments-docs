FastComments підтримує три режими автентифікації:

1. **Anonymous** -- відсутній SSO-токен; користувачі отримують ідентичності на основі сесії
2. **Simple SSO** -- токен на боці клієнта для демонстрацій та тестування (небезпечно)
3. **Secure SSO** -- підписаний сервером токен для продакшн

### Simple SSO

Підходить для демонстрацій та локального тестування. З Simple SSO будь-хто може видавати себе за будь-якого користувача, тому не використовуйте його в продакшн.

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

`SimpleSSOUserData` також підтримує додаткові поля:

- `id` -- ID користувача (за замовчуванням — email, якщо не вказано)
- `displayName` -- окреме відображуване ім'я
- `displayLabel` -- кастомна мітка, що показується поруч з ім'ям (наприклад, "VIP")
- `websiteUrl` -- посилання у імені користувача
- `locale` -- код локалі
- `isProfileActivityPrivate` -- приховати активність профілю (за замовчуванням — true)

### Secure SSO

У продакшні ваш бекенд генерує підписаний SSO-токен, використовуючи ваш API secret. iOS-додаток отримує цей токен з вашого сервера і передає його в конфіг.

**На вашому бекенді** (з використанням FastComments Swift SDK або будь-якої мови):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Поверніть цей токен вашому iOS-додатку через ваш API
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
            // Отримайте токен з вашого бекенду
            let token = try? await fetchSSOTokenFromYourBackend()
            // Створіть нову конфігурацію з токеном, або встановіть його перед завантаженням
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` підтримує додаткові поля:

- `optedInNotifications` -- підписка на email-повідомлення
- `displayLabel` -- кастомна мітка
- `displayName` -- відображуване ім'я
- `websiteUrl` -- URL вебсайту
- `groupIds` -- членство в групах
- `isAdmin` -- привілеї адміністратора
- `isModerator` -- привілеї модератора
- `isProfileActivityPrivate` -- приватність активності профілю

---
---