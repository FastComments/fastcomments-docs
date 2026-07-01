Das SDK stellt drei API‑Client‑Klassen bereit:

- **`DefaultApi`** – API‑Schlüssel‑authentifizierte Methoden für serverseitige Nutzung. Konfigurieren Sie einen API‑Schlüssel wie in [Getting Started](#getting-started-readme-generated) gezeigt.
- **`PublicApi`** – öffentliche Methoden, die keinen API‑Schlüssel benötigen und sicher aus Browsern und mobilen Apps aufgerufen werden können.
- **`ModerationApi`** – eine umfangreiche Suite von Live‑ und schnellen Moderations‑APIs. Jede `ModerationApi`‑Methode akzeptiert einen `$sso`‑Parameter und kann sich über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.

### Verwendung von PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Öffentliche Methoden erfordern keinen API-Schlüssel.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Verwendung von ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO-Payload, das den Moderator authentifiziert

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```