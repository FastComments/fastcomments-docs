The FastComments SDK предоставляет два типа конечных точек API:

### PublicAPI - конечные точки, безопасные для клиентского кода

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- Не требуют API-ключа
- Могут использовать SSO-токены для аутентификации
- Ограничены по частоте запросов для каждого пользователя/устройства
- Подходят для приложений, ориентированных на конечных пользователей

**Example use case**: Получение и создание комментариев в вашем iOS-приложении

### DefaultAPI - конечные точки для серверного кода

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- Требуют ваш API-ключ FastComments
- ДОЛЖНЫ вызываться ТОЛЬКО из серверного кода
- Обеспечивают полный доступ к данным FastComments
- Ограничены по частоте запросов на tenant

**Example use case**: Операции администратора, массовый экспорт данных, инструменты модерации

**ВАЖНО**: Никогда не выставляйте ваш API-ключ в клиентском коде. API-ключи должны использоваться только на стороне сервера.