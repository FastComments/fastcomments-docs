The SDK izpostavlja tri razrede API odjemalcev:

- **`DefaultApi`** — Metode, avtenticirane z API ključem, za uporabo na strežniku. Konfigurirajte API ključ, kot je prikazano v [Začetek](#getting-started-readme-generated).
- **`PublicApi`** — Javne metode, ki ne zahtevajo API ključa; varno jih je klicati iz brskalnikov in mobilnih aplikacij.
- **`ModerationApi`** — Metode za nadzorno ploščo moderatorja: seznam, štetje, iskanje, beleženje in izvoz komentarjev; ukrepi moderiranja (odstrani/obnovi, označi, nastavi status pregleda/spama/odobritve, glasovi, ponovno odpri/ zapri nit); prepovedi (prepoved komentiranja, razveljavitev, povzetki pred prepovedjo, stanje prepovedi in nastavitve, štetje prepovedanih uporabnikov); ter značke in zaupanje (podeli/odstrani značko, ročne značke, pridobi/nastavi faktor zaupanja, notranji profil uporabnika). Vsaka metoda `ModerationApi` sprejme parameter `$sso` za avtentikacijo delujočega moderatorja prek SSO.

### Uporaba PublicApi

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

### Uporaba ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload authenticating the moderator

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```