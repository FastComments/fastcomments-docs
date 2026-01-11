### Опростен SSO

Опростеният SSO е лесен за използване, но осигурява по-малка сигурност в сравнение със Сигурния SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Създаване на данни за потребителя
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// С URL-базирано влизане/излизане
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Или с callback функции
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Обработка на влизането */ },
    function($url) { /* Обработка на излизането */ }
);

// Вземете токена, който да предадете на FastComments
$token = $sso->prepareToSend();
```

### Сигурен SSO

Сигурният SSO предоставя повишена сигурност чрез HMAC проверка:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Създаване на данни за потребителя
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Добавете допълнителни (незадължителни) данни, ако е необходимо
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Създайте SSO обекта с вашия API ключ
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Вземете токена, който да предадете на FastComments
$token = $sso->prepareToSend();
```