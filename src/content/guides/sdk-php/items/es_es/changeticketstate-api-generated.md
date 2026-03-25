## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| userId | string | query | Sí |  |
| id | string | path | Sí |  |

## Respuesta

Devuelve: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeTicketState200Response.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de changeTicketState'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorización de la clave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomentar abajo para configurar el prefijo (p.ej., Bearer) para la clave API, si es necesario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$id = 'id_example'; // string
$change_ticket_state_body = new \FastComments\Client\Model\ChangeTicketStateBody(); // \FastComments\Client\Model\ChangeTicketStateBody

try {
    $result = $apiInstance->changeTicketState($tenant_id, $user_id, $id, $change_ticket_state_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->changeTicketState: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---