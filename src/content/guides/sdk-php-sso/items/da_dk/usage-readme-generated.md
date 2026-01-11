---
### Simpel SSO

Simpel SSO er let at bruge, men giver mindre sikkerhed end Sikker SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Opret brugerdata
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Med URL-baseret log ind/log ud
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Eller med callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Håndter log ind */ },
    function($url) { /* Håndter log ud */ }
);

// Hent tokenet, der skal sendes til FastComments
$token = $sso->prepareToSend();
```

### Sikker SSO

Sikker SSO giver forbedret sikkerhed med HMAC-verifikation:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Opret brugerdata
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Tilføj valgfri data om nødvendigt
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Opret SSO-objektet med din API-nøgle
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Hent tokenet, der skal sendes til FastComments
$token = $sso->prepareToSend();
```
---