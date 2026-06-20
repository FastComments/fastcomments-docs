---
The SDK izlaže tri API klijentske klase:

- **`DefaultApi`** — API-key-authenticated methods for server-side use. Configure an API key as shown in [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** — javne metode koje ne zahtevaju API ključ, bezbedno ih je pozivati iz pregledača i mobilnih aplikacija.
- **`ModerationApi`** — metode za moderatorski kontrolni panel: listanje, brojanje, pretraga, logovanje i izvoz komentara; moderatorske akcije (ukloni/vrati, prijavi, postavi status za pregled/šaljivinu/odobrenje, glasovi, ponovo otvori/zatvori nit); zabrane (zabrani od komentarisanja, poništi zabranu, sažeci pre-zabrane, status i podešavanja zabrane, brojevi zabranjenih korisnika); i značke & poverenje (dodeli/ukloni značku, ručne značke, dohvati/postavi faktor poverenja, interni profil korisnika). Svaka metoda `ModerationApi` prihvata parametar `$sso` za autentifikaciju moderatora putem SSO.

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
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```
---