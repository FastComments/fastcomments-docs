## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Zwraca: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetModerator200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getModerator'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Skonfiguruj autoryzację klucza API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // To jest opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getModerator($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]