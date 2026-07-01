List sider for en lejer. Bruges af FChat desktop‑klienten til at udfylde sin rumliste.  
Kræver `enableFChat` at være sand på den løste brugerdefinerede konfiguration for hver side.  
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Uigennemsigtig pagineringscursor returneret som `nextCursor` fra en tidligere forespørgsel. Knyttet til den samme `sortBy`. |
| limit | integer | query | No | 1..200, standard 50 |
| q | string | query | No | Valgfri case‑insensitiv titel‑præfiksfilter. |
| sortBy | string | query | No | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | No | Hvis sand, returneres kun sider med mindst én kommentar. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Uigennemsigtig pagineringscursor returneret som `nextCursor` fra en tidligere forespørgsel. Knyttet til den samme `sortBy`.
final limit = 56; // int | 1..200, standard 50
final q = q_example; // String | Valgfri case‑insensitiv titel‑præfiksfilter.
final sortBy = ; // PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk).
final hasComments = true; // bool | Hvis sand, returneres kun sider med mindst én kommentar.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]