## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateEmailTemplate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura la autorización de la clave API: api_key
// Descomente a continuación para configurar el prefijo (p.ej., Bearer) para la clave API, si es necesario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si deseas usar un cliente HTTP personalizado, pasa tu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará como predeterminado.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_email_template_body = new \FastComments\Client\Model\UpdateEmailTemplateBody(); // \FastComments\Client\Model\UpdateEmailTemplateBody


try {
    $result = $apiInstance->updateEmailTemplate($tenant_id, $id, $update_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]