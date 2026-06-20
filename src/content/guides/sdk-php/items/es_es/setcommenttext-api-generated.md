## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | ruta | Sí |  |
| commentId | string | ruta | Sí |  |
| broadcastId | string | consulta | Sí |  |
| editKey | string | consulta | No |  |
| sso | string | consulta | No |  |

## Respuesta

Devuelve: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicAPISetCommentTextResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de setCommentText'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$comment_text_update_request = new \FastComments\Client\Model\CommentTextUpdateRequest(); // \FastComments\Client\Model\CommentTextUpdateRequest
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->setCommentText($tenant_id, $comment_id, $broadcast_id, $comment_text_update_request, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->setCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]