## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgeProgressListResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserBadgeProgressList'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Configurar la autorización de la clave de la API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Descomente a continuación para establecer el prefijo (p. ej., Bearer) para la clave de la API, si es necesario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadgeProgressList($tenant_id, $user_id, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressList: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]