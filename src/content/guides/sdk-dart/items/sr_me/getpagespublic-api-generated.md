List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Zahtijeva da je `enableFChat` postavljeno na true u riješenoj prilagođenoj konfiguraciji za svaku stranicu.  
Stranice koje zahtijevaju SSO filtriraju se prema pristupu grupi korisnika koji napravio zahtjev.

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Veza je sa istim `sortBy`. |
| limit | integer | query | No | 1..200, podrazumijevano 50 |
| q | string | query | No | Opcionalni filter prefiksa naslova, neosjetljiv na veličinu slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (podrazumijevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice koje imaju bar jedan komentar. |

## Response

Vraća: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Primjer getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Veza je sa istim `sortBy`.
final limit = 56; // int | 1..200, podrazumijevano 50
final q = q_example; // String | Opcionalni filter prefiksa naslova, neosjetljiv na veličinu slova.
final sortBy = ; // PagesSortBy | Redoslijed sortiranja. `updatedAt` (podrazumijevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno).
final hasComments = true; // bool | Ako je true, vraća samo stranice koje imaju bar jedan komentar.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]