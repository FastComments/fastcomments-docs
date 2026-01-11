### Proste SSO

Proste SSO jest łatwe w użyciu, ale zapewnia mniejsze bezpieczeństwo niż Bezpieczne SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Utwórz dane użytkownika
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Z logowaniem/wylogowaniem opartym na adresach URL
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Lub z funkcjami zwrotnymi
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Obsłuż logowanie */ },
    function($url) { /* Obsłuż wylogowanie */ }
);

// Pobierz token do przekazania do FastComments
$token = $sso->prepareToSend();
```

### Bezpieczne SSO

Bezpieczne SSO zapewnia zwiększone bezpieczeństwo dzięki weryfikacji HMAC:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Utwórz dane użytkownika
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Dodaj opcjonalne dane, jeśli to konieczne
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Utwórz obiekt SSO z użyciem klucza API
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Pobierz token do przekazania do FastComments
$token = $sso->prepareToSend();
```