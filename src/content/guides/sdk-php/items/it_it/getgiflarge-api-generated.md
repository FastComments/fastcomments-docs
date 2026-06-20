## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| largeInternalURLSanitized | string | query | Sì |  |

## Risposta

Restituisce: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GifGetLargeResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getGifLarge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client http personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come default.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$large_internal_url_sanitized = 'large_internal_url_sanitized_example'; // string

try {
    $result = $apiInstance->getGifLarge($tenant_id, $large_internal_url_sanitized);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifLarge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]