Lista las páginas para un tenant. Usado por el cliente de escritorio FChat para poblar su lista de salas.
Requiere que `enableFChat` sea true en la configuración personalizada resuelta para cada página.
Las páginas que requieren SSO se filtran según el acceso de grupo del usuario que realiza la solicitud.

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, por defecto 50 |
| q | string | query | No | Filtro opcional de prefijo de título que no distingue entre mayúsculas y minúsculas. |
| sortBy | string | query | No | Ordenación. `updatedAt` (predeterminado, más recientes primero), `commentCount` (más comentarios primero), o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, devolver solo páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`.
$limit = 56; // int | 1..200, por defecto 50
$q = 'q_example'; // string | Filtro opcional de prefijo de título que no distingue entre mayúsculas y minúsculas.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Orden. `updatedAt` (predeterminado, más recientes primero), `commentCount` (más comentarios primero) o `title` (alfabético).
$has_comments = True; // bool | Si es true, devolver solo páginas con al menos un comentario.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]