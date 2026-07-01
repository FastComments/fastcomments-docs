List pagina’s voor een tenant. Wordt gebruikt door de FChat desktopclient om de kamerlijst te vullen.  
Vereist dat `enableFChat` true is in de opgeloste aangepaste configuratie voor elke pagina.  
Pagina’s die SSO vereisen worden gefilterd op basis van de groepsrechten van de aanvragende gebruiker.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nee | Onduidelijke pagineringscursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. |
| limit | integer | query | Nee | 1..200, standaard 50 |
| q | string | query | Nee | Optionele case‑insensitieve titelvoorvoegselfilter. |
| sortBy | string | query | Nee | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | Nee | Indien true, alleen pagina’s retourneren met ten minste één reactie. |

## Respons

Retourneert: `GetPublicPagesResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Onduidelijke pagineringscursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`.
final limit = 56; // int | 1..200, standaard 50
final q = q_example; // String | Optionele case‑insensitieve titelvoorvoegselfilter.
final sortBy = ; // PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch).
final hasComments = true; // bool | Indien true, alleen pagina’s retourneren met ten minste één reactie.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]