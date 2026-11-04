Lista las páginas de un tenant. Lo utiliza el cliente de escritorio FChat para llenar su lista de salas.
Requiere que `enableFChat` sea true en la configuración personalizada resuelta para cada página.
Las páginas que requieren SSO se filtran según el acceso a grupos del usuario solicitante.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, por defecto 50 |
| q | string | query | No | Filtro opcional de prefijo de título, insensible a mayúsculas. |
| sortBy | string | query | No | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (con más comentarios primero) o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, solo devuelve páginas con al menos un comentario. |

## Respuesta

Devuelve: `GetPublicPagesResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`.
final limit = 56; // int | 1..200, por defecto 50
final q = q_example; // String | Filtro opcional de prefijo de título, insensible a mayúsculas.
final sortBy = ; // PagesSortBy | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (con más comentarios primero) o `title` (alfabético).
final hasComments = true; // bool | Si es true, solo devuelve páginas con al menos un comentario.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]