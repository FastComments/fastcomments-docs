## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| search | string | query | Sì |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Risposta

Restituisce: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsSearchResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getGifsSearch'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` verrà usato come predefinito.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // stringa
$search = 'search_example'; // stringa
$locale = 'locale_example'; // stringa
$rating = 'rating_example'; // stringa
$page = 3.4; // float

try {
    $result = $apiInstance->getGifsSearch($tenant_id, $search, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsSearch: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]