### Jednostavan SSO

Jednostavan SSO je jednostavan za korištenje, ali pruža manju sigurnost nego Siguran SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Kreiraj podatke o korisniku
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Sa prijavom/odjavom baziranom na URL-u
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Ili sa povratnim pozivima
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Obradi prijavu */ },
    function($url) { /* Obradi odjavu */ }
);

// Dohvati token koji treba poslati FastComments
$token = $sso->prepareToSend();
```

### Siguran SSO

Siguran SSO pruža poboljšanu sigurnost uz HMAC verifikaciju:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Kreiraj podatke o korisniku
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Dodaj opcionе podatke po potrebi
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Kreiraj SSO objekt sa svojim API ključem
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Dohvati token koji treba poslati FastComments
$token = $sso->prepareToSend();
```