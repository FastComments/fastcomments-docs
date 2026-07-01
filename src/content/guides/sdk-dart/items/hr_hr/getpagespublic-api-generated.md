List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neproziran pokazivač za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan uz isti `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Opcionalni filter prefiksa naslova koji ne razlikuje veličinu slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice s najmanje jednim komentarom. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Primjer getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Neproziran pokazivač za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan uz isti `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | Opcionalni filter prefiksa naslova koji ne razlikuje veličinu slova.
final sortBy = ; // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno).
final hasComments = true; // bool | Ako je true, vraća samo stranice s najmanje jednim komentarom.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]