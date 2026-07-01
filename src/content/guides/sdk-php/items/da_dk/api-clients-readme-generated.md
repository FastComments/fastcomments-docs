The SDK'en eksponerer tre API-klientklasser:

- **`DefaultApi`** - API‑nøgle‑autentificerede metoder til server‑side brug. Konfigurér en API‑nøgle som vist i [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - offentlige metoder, der ikke kræver en API‑nøgle, sikre at kalde fra browsere og mobilapps.
- **`ModerationApi`** - en omfattende suite af live og hurtige moderations‑API'er. Hver `ModerationApi`‑metode accepterer en `$sso`‑parameter og kan autentificere via SSO eller en FastComments.com sessions‑cookie.

### Brug af PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Offentlige metoder kræver ikke en API-nøgle.
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

### Brug af ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO-indhold, der autentificerer moderatoren

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```