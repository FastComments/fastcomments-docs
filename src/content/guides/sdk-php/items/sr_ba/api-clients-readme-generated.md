---
SDK izlaže tri API klijentske klase:

- **`DefaultApi`** — Metode koje koriste autentifikaciju API ključem za upotrebu na serverskoj strani. Konfigurišite API ključ kao što je prikazano u [Početak rada](#getting-started-readme-generated).
- **`PublicApi`** — javne metode koje ne zahtijevaju API ključ, sigurne za pozivanje iz preglednika i mobilnih aplikacija.
- **`ModerationApi`** — metode za moderatorsku kontrolnu ploču: listanje, brojanje, pretraživanje, logovanje i izvoz komentara; akcije moderacije (uklanjanje/obnavljanje, označavanje, postavljanje statusa za pregled/spam/odobrenje, glasovi, ponovno otvaranje/zatvaranje teme); zabrane (zabrana komentarisanja, poništavanje, sažeci prije zabrane, status zabrane i preferencije, broj zabranjenih korisnika); i značke i povjerenje (dodjela/uklanjanje značke, ručne značke, dohvatanje/postavljanje faktora povjerenja, interni korisnički profil). Svaka `ModerationApi` metoda prihvata `$sso` parametar za autentifikaciju moderatora putem SSO.

### Korištenje PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Javne metode ne zahtijevaju API ključ.
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
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```
---