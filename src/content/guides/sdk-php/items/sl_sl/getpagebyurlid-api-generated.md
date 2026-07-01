## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Odgovor

Vrne: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPageByURLIdAPIResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getPageByURLId'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Konfigurirajte avtorizacijo ključa API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za ključ API, po potrebi
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Če želite uporabiti prilagojeni HTTP odjemalec, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string


try {
    $result = $apiInstance->getPageByURLId($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPageByURLId: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]