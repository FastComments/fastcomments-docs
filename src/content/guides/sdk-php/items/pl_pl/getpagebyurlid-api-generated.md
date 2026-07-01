## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | query | Tak |  |
| urlId | string | query | Tak |  |

## Response

Zwraca: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPageByURLIdAPIResponse.php)

## Example

[inline-code-attrs-start title = 'Przykład getPageByURLId'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Konfiguruj autoryzację klucza API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebny
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Jeśli chcesz używać własnego klienta http, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // To jest opcjonalne, `GuzzleHttp\Client` będzie używany jako domyślny.
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