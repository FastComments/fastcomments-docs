### Једноставни SSO

Једноставни SSO је једноставан за коришћење, али пружа мање безбедности него Сигуран SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Креирајте корисничке податке
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Са пријављивањем/одјављивањем заснованим на URL-у
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Или помоћу повратних функција
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Handle login */ },
    function($url) { /* Handle logout */ }
);

// Добијте токен који треба проследити FastComments-у
$token = $sso->prepareToSend();
```

### Сигуран SSO

Сигуран SSO пружа појачану безбедност уз HMAC верификацију:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Креирајте корисничке податке
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Додајте опционе податке по потреби
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Креирајте SSO објекат са вашим API кључем
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Добијте токен који треба проследити FastComments-у
$token = $sso->prepareToSend();
```