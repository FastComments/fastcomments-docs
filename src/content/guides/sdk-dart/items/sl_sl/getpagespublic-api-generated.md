Navede strani za najemnika. Uporablja namizni odjemalec FChat za izpolnitev svojega seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop do skupin zahtevajočega uporabnika.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kazalec za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. |
| sortBy | string | query | Ne | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). |
| hasComments | boolean | query | Ne | Če je true, vrni samo strani z vsaj enim komentarjem. |

## Odziv

Vrne: `GetPublicPagesResponse`

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Neprozoren kazalec za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`.
final limit = 56; // int | 1..200, privzeto 50
final q = q_example; // String | Neobvezen filter predpone naslova, neobčutljiv na velikost črk.
final sortBy = ; // PagesSortBy | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno).
final hasComments = true; // bool | Če je true, vrni samo strani z vsaj enim komentarjem.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]