List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque renvoyé comme `nextCursor` d’une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, par défaut 50 |
| q | string | query | No | Filtre optionnel de préfixe de titre insensible à la casse. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, le plus récent d'abord), `commentCount` (le plus de commentaires d'abord) ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si true, ne renvoie que les pages contenant au moins un commentaire. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Curseur de pagination opaque renvoyé comme `nextCursor` d’une requête précédente. Lié au même `sortBy`.
final limit = 56; // int | 1..200, par défaut 50
final q = q_example; // String | Filtre optionnel de préfixe de titre insensible à la casse.
final sortBy = ; // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, le plus récent d'avant), `commentCount` (le plus de commentaires d'avant) ou `title` (alphabétique).
final hasComments = true; // bool | Si true, ne renvoie que les pages contenant au moins un commentaire.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]