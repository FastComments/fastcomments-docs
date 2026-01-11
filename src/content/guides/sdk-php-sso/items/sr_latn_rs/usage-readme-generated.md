### Jednostavan SSO

Jednostavan SSO je jednostavan za upotrebu, ali pruža manju bezbednost od Zaštićenog SSO-a:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Kreiraj podatke o korisniku
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Sa prijavom/odjavom zasnovanom na URL-u
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

// Dobij token koji treba proslediti FastComments-u
$token = $sso->prepareToSend();
```

### Zaštićeni SSO

Zaštićeni SSO obezbeđuje pojačanu sigurnost pomoću HMAC verifikacije:

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

// Kreiraj SSO objekat sa vašim API ključem
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Dobij token koji treba proslediti FastComments-u
$token = $sso->prepareToSend();
```