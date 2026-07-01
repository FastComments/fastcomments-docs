List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren krmarilni kazalec, vrnjen kot `nextCursor` iz prejšnje zahteve. Vezen na enak `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezni filter predpone naslova, ki ne upošteva velikosti črk. |
| sortBy | string | query | Ne | Vrstni red. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abeceden). |
| hasComments | boolean | query | Ne | Če je true, vrne le strani z vsaj enim komentarjem. |

## Odgovor

Vrne: `GetPublicPagesResponse`

## Primer

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Neprozoren krmarilni kazalec, vrnjen kot `nextCursor` iz prejšnje zahteve. Vezen na enak `sortBy`.
final limit = 56; // int | 1..200, privzeto 50
final q = q_example; // String | Neobvezni filter predpone naslova, ki ne upošteva velikosti črk.
final sortBy = ; // PagesSortBy | Vrstni red. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abeceden).
final hasComments = true; // bool | Če je true, vrne le strani z vsaj enim komentarjem.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]