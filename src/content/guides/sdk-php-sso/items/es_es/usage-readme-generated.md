### SSO simple

SSO simple es fácil de usar, pero proporciona menos seguridad que SSO seguro:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Crear datos de usuario
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Con inicio/cierre de sesión basado en URL
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// O con callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Gestionar inicio de sesión */ },
    function($url) { /* Gestionar cierre de sesión */ }
);

// Obtener el token para pasar a FastComments
$token = $sso->prepareToSend();
```

### SSO seguro

SSO seguro proporciona mayor seguridad con verificación HMAC:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Crear datos de usuario
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Agregar datos opcionales si es necesario
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Crear el objeto SSO con tu clave API
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Obtener el token para pasar a FastComments
$token = $sso->prepareToSend();
```