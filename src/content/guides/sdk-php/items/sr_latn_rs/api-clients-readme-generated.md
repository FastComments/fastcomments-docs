The SDK izlaže tri klase klijenta API-ja:

- **`DefaultApi`** - Metode autentifikovane API ključem za upotrebu na serveru. Konfigurišite API ključ kako je prikazano u [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - Javne metode koje ne zahtevaju API ključ, bezbedne za pozivanje iz pregledača i mobilnih aplikacija.
- **`ModerationApi`** - Opsežni skup uživo i brzih moderacijskih API-ja. Svaka metoda `ModerationApi` prihvata parametar `$sso` i može se autentifikovati putem SSO-a ili sesijskog kolačića FastComments.com.

### Korišćenje PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Javne metode ne zahtevaju API ključ.
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
$sso = 'sso_example'; // string - SSO payload koji autentifikuje moderatora

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```