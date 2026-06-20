---
## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| updateComments | string | query | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateTenantUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar la autorización de la clave de API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente lo siguiente para configurar el prefijo (p. ej. Bearer) para la clave de API, si es necesario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_tenant_user_body = new \FastComments\Client\Model\UpdateTenantUserBody(); // \FastComments\Client\Model\UpdateTenantUserBody
$update_comments = 'update_comments_example'; // string

try {
    $result = $apiInstance->updateTenantUser($tenant_id, $id, $update_tenant_user_body, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---