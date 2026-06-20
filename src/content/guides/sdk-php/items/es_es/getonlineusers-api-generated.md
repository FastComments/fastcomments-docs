Actualmente espectadores en línea de una página: personas cuya sesión websocket está suscrita a la página en este momento.
Devuelve anonCount + totalCount (suscriptores en toda la sala, incluyendo espectadores anónimos que no enumeramos).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de la URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no hagan que se omitan entradas. |

## Response

Devuelve: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará como valor predeterminado.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificador de la URL de la página (limpiado en el servidor).
$after_name = 'after_name_example'; // string | Cursor: pase nextAfterName de la respuesta anterior.
$after_user_id = 'after_user_id_example'; // string | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName esté establecido para que los empates por nombre no hagan que se omitan entradas.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]