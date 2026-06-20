## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetModeratorsResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getModerators'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Configurar autorización de clave API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Descomente abajo para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
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
$skip = 3.4; // float

try {
    $result = $apiInstance->getModerators($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getModerators: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]