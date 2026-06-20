## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| contextUserId | string | query | Ne |  |
| isLive | boolean | query | Ne |  |

## Odgovor

Vraća: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentResult.php)

## Primer

[inline-code-attrs-start title = 'Primer za deleteComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguriši autorizaciju API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Otkomentari ispod da postaviš prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želiš da koristiš prilagođeni HTTP klijent, prosledi klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` biće korišćen kao podrazumevani.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$context_user_id = 'context_user_id_example'; // string
$is_live = True; // bool

try {
    $result = $apiInstance->deleteComment($tenant_id, $id, $context_user_id, $is_live);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]