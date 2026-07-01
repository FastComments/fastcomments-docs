## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Odgovor

Returns: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddPageAPIResponse.php)

## Primer

[inline-code-attrs-start title = 'addPage Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Konfiguriraj avtentikacijo API ključa: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentiraj spodaj za nastavitev predpon (npr. Bearer) za API ključ, po potrebi
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Če želite uporabiti lasten HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// niz
$create_api_page_data = new \FastComments\Client\Model\CreateAPIPageData(); // \FastComments\Client\Model\CreateAPIPageData


try {
    $result = $apiInstance->addPage($tenant_id, $create_api_page_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addPage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]