Izlistava stranice za tenant. Koristi se od strane FChat desktop klijenta za popunjavanje liste soba.  
Zahteva da `enableFChat` bude postavljen na true u razrešenom prilagođenom konfiguraciji za svaku stranicu.  
Stranice koje zahtevaju SSO se filtriraju u odnosu na grupni pristup korisnika koji zahteva.

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahteva. Povezan je sa istim `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcioni filter prefiksa naslova neosetljiv na veličinu slova. |
| sortBy | string | query | No | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice koje imaju bar jedan komentar. |

## Response

Vraća: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | Optional case-insensitive title prefix filter.
final sortBy = ; // PagesSortBy | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical).
final hasComments = true; // bool | If true, only return pages with at least one comment.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]