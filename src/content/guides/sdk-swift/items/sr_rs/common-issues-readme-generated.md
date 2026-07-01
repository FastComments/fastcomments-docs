### 401 Неовлашћене грешке

Ако добијате 401 грешке приликом коришћења аутентификованог API-ја:

1. **Проверите ваш API кључ**: Уверите се да користите тачан API кључ са вашег FastComments контролног панела
2. **Проверите tenant ID**: Уверите се да ID тенанта одговара вашем налогу
3. **API key format**: API кључ треба да буде постављен као `x-api-key` заглавље у заједничкој конфигурацији:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Using the wrong API**: Уверите се да користите `DefaultAPI` (а не `PublicAPI`) за аутентификоване позиве

### Проблеми са SSO токенима

Ако SSO токени не раде:

1. **Use secure mode for production**: Увек користите `FastCommentsSSO.createSecure()` са вашим API кључем за продукцију
2. **Server-side only**: Генеришите безбедне SSO токене на вашем серверу, никада не излагайте ваш API кључ клијентима
3. **Check user data**: Уверите се да су сви потребни поља (id, email, username) достављени
4. **Token expiration**: Безбедни SSO токени садрже временски печат и могу истећи. Генеришите нове токене по потреби.

### SSL/TLS грешке

Ако наиђете на SSL/TLS грешке:

1. Уверите се да ваш Info.plist дозвољава HTTPS везе ка fastcomments.com
2. Проверите да не користите искључења App Transport Security која могу блокирати везу