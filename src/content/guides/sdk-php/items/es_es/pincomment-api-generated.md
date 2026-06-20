## Parámetros

| Name | Type | Location | Obligatorio | Descripción |
|------|------|----------|------------|-------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| broadcastId | string | query | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeCommentPinStatusResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de pinComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se utilizará por defecto.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->pinComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->pinComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]