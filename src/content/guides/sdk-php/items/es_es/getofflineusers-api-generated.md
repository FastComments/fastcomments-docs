Comentaristas anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Úselo después de agotar /users/online para mostrar una sección "Miembros".
Paginación por cursor sobre commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName}
desde afterName hacia adelante mediante $gt, sin coste de $skip.

## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Obligatorio cuando afterName está establecido para que los empates de nombre no descarten entradas. |

## Respuesta

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente http personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificador de URL de la página (limpiado en el servidor).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: pase nextAfterName de la respuesta anterior.
    'after_user_id' => 'after_user_id_example', // string | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Obligatorio cuando afterName está establecido para que los empates de nombre no descarten entradas.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]