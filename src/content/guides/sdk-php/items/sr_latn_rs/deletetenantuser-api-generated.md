## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| deleteComments | string | query | Ne |  |
| commentDeleteMode | string | query | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer deleteTenantUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguriši autorizaciju API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Otkomentariši ispod da postaviš prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želiš da koristiš prilagođeni HTTP klijent, prosledi svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će biti korišćen kao podrazumevani.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$delete_comments = 'delete_comments_example'; // string
$comment_delete_mode = 'comment_delete_mode_example'; // string

try {
    $result = $apiInstance->deleteTenantUser($tenant_id, $id, $delete_comments, $comment_delete_mode);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]