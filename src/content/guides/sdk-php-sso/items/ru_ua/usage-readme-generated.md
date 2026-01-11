### Простой SSO

Простой SSO прост в использовании, но обеспечивает меньшую безопасность, чем Защищённый SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Создаём данные пользователя
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// С входом/выходом по URL
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Или с обратными вызовами
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Обработать вход */ },
    function($url) { /* Обработать выход */ }
);

// Получите токен, который нужно передать FastComments
$token = $sso->prepareToSend();
```

### Защищённый SSO

Защищённый SSO предоставляет повышенную безопасность с HMAC-проверкой:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Создаём данные пользователя
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Добавьте дополнительные данные при необходимости
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Создайте объект SSO с вашим API-ключом
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Получите токен, который нужно передать FastComments
$token = $sso->prepareToSend();
```