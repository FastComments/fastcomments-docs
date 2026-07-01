List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | Ne | 1..200, podrazumevano 50 |
| q | string | query | Ne | Opcionalni filter prefiksa naslova neosetljivog na veličinu slova. |
| sortBy | string | query | Ne | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraća samo stranice sa najmanje jednim komentarom. |

## Response

Vraća: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`.
final limit = 56; // int | 1..200, podrazumevano 50
final q = q_example; // String | Opcionalni filter prefiksa naslova neosetljivog na veličinu slova.
final sortBy = ; // PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno).
final hasComments = true; // bool | Ako je true, vraća samo stranice sa najmanje jednim komentarom.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]