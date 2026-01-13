### SSO simple

Le SSO simple est facile à utiliser, mais offre moins de sécurité que le SSO sécurisé :

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Créer les données utilisateur
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Avec connexion/déconnexion via URL
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Ou avec des callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Gérer la connexion */ },
    function($url) { /* Gérer la déconnexion */ }
);

// Récupérer le jeton à transmettre à FastComments
$token = $sso->prepareToSend();
```

### SSO sécurisé

Le SSO sécurisé offre une sécurité renforcée grâce à la vérification HMAC :

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Créer les données utilisateur
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Ajouter des données optionnelles si nécessaire
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Créer l'objet SSO avec votre clé API
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Récupérer le jeton à transmettre à FastComments
$token = $sso->prepareToSend();
```