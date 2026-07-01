The SDK biedt drie API‑clientklassen:

- **`DefaultApi`** – API‑sleutel‑geauthenticeerde methoden voor server‑side gebruik. Configureer een API‑sleutel zoals weergegeven in [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** – openbare methoden die geen API‑sleutel vereisen, veilig om aan te roepen vanuit browsers en mobiele apps.
- **`ModerationApi`** – een uitgebreide reeks live en snelle moderatie‑API’s. Elke `ModerationApi`‑methode accepteert een `$sso`‑parameter en kan authenticeren via SSO of een FastComments.com‑sessie‑cookie.

### PublicApi gebruiken

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Publieke methoden vereisen geen API-sleutel.
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

### ModerationApi gebruiken

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload die de moderator authenticeert

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```