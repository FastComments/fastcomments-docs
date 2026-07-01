List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

Lista las páginas de un inquilino. Utilizado por el cliente de escritorio FChat para rellenar su lista de salas.  
Requiere que `enableFChat` sea verdadero en la configuración personalizada resuelta para cada página.  
Las páginas que requieren SSO se filtran según el acceso grupal del usuario que solicita.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | ruta | Sí |  |
| cursor | string | consulta | No | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud previa. Unido al mismo `sortBy`. |
| limit | integer | consulta | No | 1..200, default 50 |
| q | string | consulta | No | Filtro opcional de prefijo de título sin distinción de mayúsculas/minúsculas. |
| sortBy | string | consulta | No | Orden de clasificación. `updatedAt` (por defecto, más reciente primero), `commentCount` (más comentarios primero), o `title` (alfabético). |
| hasComments | boolean | consulta | No | Si es verdadero, solo devuelve páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si deseas usar un cliente http personalizado, pasa tu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se utilizará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud previa. Unido al mismo `sortBy`.
    'limit' => 56, // int | 1..200, default 50
    'q' => 'q_example', // string | Filtro opcional de prefijo de título sin distinción de mayúsculas/minúsculas.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Orden de clasificación. `updatedAt` (por defecto, más reciente primero), `commentCount` (más comentarios primero), o `title` (alfabético).
    'has_comments' => True, // bool | Si es verdadero, solo devuelve páginas con al menos un comentario.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]