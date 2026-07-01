L'SDK espone tre classi client API:

- **`DefaultApi`** - metodi autenticati con chiave API per uso lato server. Configura una chiave API come mostrato in [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - metodi pubblici che non richiedono una chiave API, sicuri da chiamare da browser e app mobili.
- **`ModerationApi`** - una suite completa di API di moderazione in tempo reale e veloce. Ogni metodo `ModerationApi` accetta un parametro `$sso` e può autenticarsi tramite SSO o un cookie di sessione FastComments.com.

### Utilizzo di PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// I metodi pubblici non richiedono una chiave API.
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

### Utilizzo di ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - Payload SSO che autentica il moderatore

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```