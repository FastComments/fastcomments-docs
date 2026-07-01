The SDK izlaže tri klase API klijenta:

- **`DefaultApi`** – metode autentifikovane API‑ključem za server‑side upotrebu. Konfigurišite API ključ kao što je prikazano u [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** – javne metode koje ne zahtijevaju API ključ, sigurne za pozivanje iz preglednika i mobilnih aplikacija.
- **`ModerationApi`** – opširni paket API‑ja za moderaciju u realnom vremenu i brzu moderaciju. Svaka metoda `ModerationApi` prima `$sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.

### Korišćenje PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public methods do not require an API key.
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

### Korišćenje ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload authenticating the moderator

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```