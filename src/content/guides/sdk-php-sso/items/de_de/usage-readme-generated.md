### Einfaches SSO

Einfaches SSO ist einfach zu verwenden, bietet jedoch weniger Sicherheit als Sicheres SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Benutzerdaten erstellen
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Mit URL-basiertem Login/Logout
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Oder mit Callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Anmeldung verarbeiten */ },
    function($url) { /* Abmeldung verarbeiten */ }
);

// Token erhalten, das an FastComments übergeben wird
$token = $sso->prepareToSend();
```

### Sicheres SSO

Sicheres SSO bietet erhöhte Sicherheit durch HMAC-Überprüfung:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Benutzerdaten erstellen
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Optionale Daten bei Bedarf hinzufügen
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// SSO-Objekt mit Ihrem API-Schlüssel erstellen
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Token erhalten, das an FastComments übergeben wird
$token = $sso->prepareToSend();
```