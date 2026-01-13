### 401 Помилки неавторизованого доступу

Якщо ви отримуєте помилки 401 при використанні автентифікованого API:

1. **Check your API key**: Переконайтеся, що ви використовуєте правильний API key з вашої панелі керування FastComments
2. **Verify the tenant ID**: Переконайтеся, що tenant ID відповідає вашому обліковому запису
3. **API key format**: API key має бути встановлений на API client:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Using the wrong API**: Переконайтеся, що ви використовуєте `DefaultAPI` (не `PublicAPI`) для автентифікованих викликів

### Проблеми з SSO токенами

Якщо SSO токени не працюють:

1. **Use secure mode for production**: Завжди використовуйте `FastCommentsSSO.createSecure()` з вашим API key для продакшену
2. **Server-side only**: Генеруйте захищені SSO токени на вашому сервері, ніколи не розголошуйте ваш API key клієнтам
3. **Check user data**: Переконайтесь, що всі необхідні поля (id, email, username) надані
4. **Token expiration**: Secure SSO токени містять мітку часу і можуть закінчуватися. Генеруйте нові токени за потреби.

### Помилки SSL/TLS

Якщо ви стикаєтеся з помилками SSL/TLS:

1. Переконайтеся, що Info.plist вашого додатка дозволяє HTTPS-підключення до fastcomments.com
2. Переконайтеся, що ви не використовуєте винятки App Transport Security, які можуть блокувати підключення