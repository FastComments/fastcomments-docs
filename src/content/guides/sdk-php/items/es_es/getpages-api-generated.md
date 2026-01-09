## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPagesAPIResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar la autorización de la clave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente abajo para configurar el prefijo (p. ej. Bearer) para la clave API, si es necesario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional; se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string

try {
    $result = $apiInstance->getPages($tenant_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]