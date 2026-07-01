List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Nevidljiv (opaque) kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan je sa istim `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcionalni filter prefiksa naslova koji ne razlikuje velika i mala slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (podrazumevano, najnoviji prvo), `commentCount` (najviše komentara prvo), ili `title` (alfabetski). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice koje imaju bar jedan komentar. |

## Response

Returns: `GetPublicPagesResponse`

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Nevidljiv (opaque) kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan je sa istim `sortBy`.
final limit = 56; // int | 1..200, podrazumevano 50
final q = q_example; // String | Opcionalni filter prefiksa naslova koji ne razlikuje velika i mala slova.
final sortBy = ; // PagesSortBy | Redoslijed sortiranja. `updatedAt` (podrazumevano, najnoviji prvo), `commentCount` (najviše komentara prvo), ili `title` (alfabetski).
final hasComments = true; // bool | Ako je true, vraća samo stranice koje imaju bar jedan komentar.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]