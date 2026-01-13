### Jednostavni SSO

Simple SSO je jednostavan za korištenje, ali pruža manje sigurnosti nego Siguran SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Kreiraj podatke o korisniku
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// S prijavom/odjavom putem URL-a
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Ili s callback funkcijama
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Obradi prijavu */ },
    function($url) { /* Obradi odjavu */ }
);

// Dohvati token za prosljeđivanje FastCommentsu
$token = $sso->prepareToSend();
```

### Siguran SSO

Secure SSO pruža poboljšanu sigurnost s HMAC provjerom:

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

// Dodaj opcionalne podatke po potrebi
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Kreiraj SSO objekt s vašim API ključem
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Dohvati token za prosljeđivanje FastCommentsu
$token = $sso->prepareToSend();
```