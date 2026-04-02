---
FastComments поддерживает три режима аутентификации:

1. **Анонимный** -- без SSO-токена; пользователи получают идентичности на основе сессии
2. **Простой SSO** -- токен на стороне клиента для демонстраций и тестирования (небезопасно)
3. **Защищённый SSO** -- токен, подписанный сервером, для боевого использования

### Простой SSO

Полезен для демонстраций и локального тестирования. С помощью Простой SSO любой может выдавать себя за любого пользователя, поэтому не используйте его в продакшене.

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

`SimpleSSOUserData` также поддерживает дополнительные необязательные поля:

- `id` -- идентификатор пользователя (по умолчанию email, если не указан)
- `displayName` -- отдельное отображаемое имя
- `displayLabel` -- пользовательская метка, показываемая рядом с именем (например, "VIP")
- `websiteUrl` -- ссылка на имени пользователя
- `locale` -- код локали
- `isProfileActivityPrivate` -- скрыть активность в профиле (по умолчанию true)

### Защищённый SSO

В продакшене ваш бэкенд генерирует подписанный SSO-токен, используя ваш API-секрет. iOS-приложение запрашивает этот токен у сервера и передаёт его в конфигурацию.

**На вашем бэкенде** (используя FastComments Swift SDK или любой другой язык):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Верните этот токен в ваше iOS-приложение через ваш API
```

**В вашем iOS-приложении:**

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
            // Получить токен с вашего бэкенда
            let token = try? await fetchSSOTokenFromYourBackend()
            // Создайте новый config с токеном или установите его до загрузки
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` поддерживает дополнительные поля:

- `optedInNotifications` -- подписка на уведомления по email
- `displayLabel` -- пользовательская метка
- `displayName` -- отображаемое имя
- `websiteUrl` -- URL веб-сайта
- `groupIds` -- членство в группах
- `isAdmin` -- привилегии администратора
- `isModerator` -- привилегии модератора
- `isProfileActivityPrivate` -- приватность активности профиля

---
---