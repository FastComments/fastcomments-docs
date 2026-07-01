The SDK razkriva tri razrede odjemalcev API:

- **`DefaultApi`** - metode, avtenticirane s ključem API za strežniško uporabo. Ključ API nastavite, kot je prikazano v [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - javne metode, ki ne zahtevajo ključa API, varno jih je klicati iz brskalnikov in mobilnih aplikacij.
- **`ModerationApi`** - obsežen nabor API-jev za takojšnjo in hitro moderacijo. Vsaka metoda `ModerationApi` sprejme parameter `$sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.

### Uporaba PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Javni metode ne zahtevajo ključa API.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // niz
$url_id = 'url_id_example'; // niz

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Uporaba ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // niz - SSO payload, ki avtenticira moderatorja

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```