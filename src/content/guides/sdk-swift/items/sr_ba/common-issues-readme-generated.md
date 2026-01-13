### 401 Неовлашћене грешке

Ако добијате 401 грешке приликом кориштења аутентификованог API-ја:

1. **Check your API key**: Осигурајте се да користите тачан API key са ваше FastComments контролне табле
2. **Verify the tenant ID**: Убедите се да tenant ID одговара вашем налогу
3. **API key format**: The API key should be set on the API client:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Using the wrong API**: Осигурајте се да користите `DefaultAPI` (не `PublicAPI`) за аутентификоване позиве

### Проблеми са SSO токеном

Ако SSO токени не функционишу:

1. **Use secure mode for production**: Увек користите `FastCommentsSSO.createSecure()` са вашим API key-ом за продукцију
2. **Server-side only**: Генеришите сигурне SSO токене на вашем серверу, никада не откривајте ваш API key клијентима
3. **Check user data**: Осигурајте да су сва обавезна поља (id, email, username) обезбеђена
4. **Token expiration**: Сигурни SSO токени садрже временску ознаку и могу истећи. Генеришите нове токене по потреби.

### SSL/TLS грешке

Ако наиђете на SSL/TLS грешке:

1. Осигурајте да Info.plist ваше апликације дозвољава HTTPS везе према fastcomments.com
2. Провјерите да не користите App Transport Security exceptions које би могле блокирати везу