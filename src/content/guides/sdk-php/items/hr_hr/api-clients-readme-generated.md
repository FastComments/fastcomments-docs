SDK izlaže tri klase API klijenata:

- **`DefaultApi`** — metode autentificirane API ključem za upotrebu na poslužiteljskoj strani. Konfigurirajte API ključ kao što je prikazano u [Početak rada](#getting-started-readme-generated).
- **`PublicApi`** — javne metode koje ne zahtijevaju API ključ, sigurne za pozivanje iz preglednika i mobilnih aplikacija.
- **`ModerationApi`** — metode za nadzornu ploču moderatora: popisivanje, brojanje, pretraživanje, evidentiranje i izvoz komentara; akcije moderiranja (ukloni/vrati, označi/prijavi, postavi status pregled/spam/odobrenje, glasovi, ponovno otvorenje/zatvaranje nita); zabrane (zabrana komentiranja, poništi, sažeci prije zabrane, status i postavke zabrane, brojevi zabranjenih korisnika); te značke i povjerenje (dodijeli/ukloni značku, ručne značke, dohvati/postavi faktor povjerenja, unutarnji profil korisnika). Svaka metoda `ModerationApi` prihvaća parametar `$sso` za autentifikaciju moderatora putem SSO.

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