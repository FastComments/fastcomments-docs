### Simple SSO

Simple SSO is eenvoudig te gebruiken, maar biedt minder beveiliging dan Secure SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Gebruikersgegevens aanmaken
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Met URL-gebaseerde login/logout
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Of met callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Login afhandelen */ },
    function($url) { /* Logout afhandelen */ }
);

// Haal de token op om door te geven aan FastComments
$token = $sso->prepareToSend();
```

### Secure SSO

Secure SSO biedt verbeterde beveiliging met HMAC-verificatie:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Gebruikersgegevens aanmaken
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Optionele gegevens toevoegen indien nodig
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Maak het SSO-object aan met je API-sleutel
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Haal de token op om door te geven aan FastComments
$token = $sso->prepareToSend();
```