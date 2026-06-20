---
Comentaristas anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Utilice esto después de agotar /users/online para mostrar una sección "Miembros".
Paginación por cursor en commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName}
índice desde afterName hacia adelante mediante $gt, sin coste de $skip.

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Respuesta

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificador de la URL de la página (limpiado en el servidor).
$after_name = 'after_name_example'; // string | Cursor: pasar nextAfterName de la respuesta anterior.
$after_user_id = 'after_user_id_example'; // string | Desempate del cursor: pasar nextAfterUserId de la respuesta anterior. Requerido cuando afterName esté establecido para que los empates por nombre no descarten entradas.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---