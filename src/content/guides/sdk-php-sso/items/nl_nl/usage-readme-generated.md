### Simple SSO

Simple SSO is eenvoudig te gebruiken, maar biedt minder beveiliging dan Secure SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Maak gebruikersgegevens aan
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Met URL-gebaseerde aanmelding/afmelding
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Of met callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Verwerk login */ },
    function($url) { /* Verwerk logout */ }
);

// Haal het token op om aan FastComments door te geven
$token = $sso->prepareToSend();
```

### Secure SSO

Secure SSO biedt verbeterde beveiliging met HMAC-verificatie:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Maak gebruikersgegevens aan
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Voeg optionele gegevens toe indien nodig
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Maak het SSO-object met uw API-sleutel
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Haal het token op om aan FastComments door te geven
$token = $sso->prepareToSend();
```