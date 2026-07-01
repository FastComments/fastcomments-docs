The SDK izlaže tri klase API klijenta:

- **`DefaultApi`** – metode autentificirane API ključem za korištenje na poslužitelju. Konfigurirajte API ključ kako je prikazano u [Početak rada](#getting-started-readme-generated).
- **`PublicApi`** – javne metode koje ne zahtijevaju API ključ, sigurne za pozivanje iz preglednika i mobilnih aplikacija.
- **`ModerationApi`** – opsežan skup API‑ja za moderiranje u stvarnom vremenu i brzu moderaciju. Svaka metoda `ModerationApi` prihvaća `$sso` parametar i može se autentificirati putem SSO‑a ili FastComments.com kolačića sesije.

### Korištenje PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Javni metodi ne zahtijevaju API ključ.
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

### Korištenje ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload koji autentificira moderatora

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
}
```