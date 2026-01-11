### 401 Неоторизирани грешки

If you're getting 401 errors when using the authenticated API:

1. **Check your API key**: Уверете се, че използвате правилния API ключ от таблото за управление на FastComments
2. **Verify the tenant ID**: Уверете се, че tenant ID съвпада с вашия акаунт
3. **API key format**: API ключът трябва да бъде зададен в API клиента:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Using the wrong API**: Уверете се, че използвате `DefaultAPI` (не `PublicAPI`) за автентифицирани повиквания

### Проблеми със SSO токените

If SSO tokens aren't working:

1. **Use secure mode for production**: Винаги използвайте `FastCommentsSSO.createSecure()` с вашия API ключ за производствена среда
2. **Server-side only**: Генерирайте сигурни SSO токени на вашия сървър, никога не излагайте API ключа си на клиентите
3. **Check user data**: Уверете се, че всички задължителни полета (id, email, username) са предоставени
4. **Token expiration**: Сигурните SSO токени включват времеви печат и могат да изтекат. Генерирайте нови токени при необходимост.

### SSL/TLS грешки

If you encounter SSL/TLS errors:

1. Уверете се, че Info.plist на вашето приложение позволява HTTPS връзки към fastcomments.com
2. Проверете, че не използвате изключения на App Transport Security, които биха могли да блокират връзката