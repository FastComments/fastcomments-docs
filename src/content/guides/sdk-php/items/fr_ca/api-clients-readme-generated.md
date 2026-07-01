Le SDK expose trois classes client d’API :

- **`DefaultApi`** – Méthodes authentifiées par clé API pour une utilisation côté serveur. Configurez une clé API comme indiqué dans [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** – Méthodes publiques qui ne nécessitent pas de clé API, sécurisées pour être appelées depuis les navigateurs et les applications mobiles.
- **`ModerationApi`** – Une suite étendue d’API de modération en temps réel et rapides. Chaque méthode `ModerationApi` accepte un paramètre `$sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.

### Utilisation de PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Les méthodes publiques ne nécessitent pas de clé API.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // chaîne
$url_id = 'url_id_example'; // chaîne

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Utilisation de ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // chaîne - charge utile SSO authentifiant le modérateur

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```