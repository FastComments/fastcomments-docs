### Preprost SSO

Preprost SSO je preprost za uporabo, vendar zagotavlja manj varnosti kot Varen SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Ustvari podatke uporabnika
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Z URL-podprto prijavo/odjavo
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Ali z povratnimi klici
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Obdelaj prijavo */ },
    function($url) { /* Obdelaj odjavo */ }
);

// Pridobi žeton za posredovanje FastComments
$token = $sso->prepareToSend();
```

### Varen SSO

Varen SSO zagotavlja izboljšano varnost z HMAC preverjanjem:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Ustvari podatke uporabnika
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Dodaj neobvezne podatke, če je potrebno
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Ustvari SSO objekt z vašim API ključem
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Pridobi žeton za posredovanje FastComments
$token = $sso->prepareToSend();
```