List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, predeterminado 50 |
| q | string | query | No | Filtro opcional de prefijo de título que no distingue entre mayúsculas y minúsculas. |
| sortBy | string | query | No | Orden de clasificación. `updatedAt` (predeterminado, más reciente primero), `commentCount` (más comentarios primero), o `title` (alfabético). |
| hasComments | boolean | query | No | Si es verdadero, solo devuelve páginas con al menos un comentario. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Ejemplo getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`.
final limit = 56; // int | 1..200, predeterminado 50
final q = q_example; // String | Filtro opcional de prefijo de título que no distingue entre mayúsculas y minúsculas.
final sortBy = ; // PagesSortBy | Orden de clasificación. `updatedAt` (predeterminado, más reciente primero), `commentCount` (más comentarios primero), o `title` (alfabético).
final hasComments = true; // bool | Si es verdadero, solo devuelve páginas con al menos un comentario.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]