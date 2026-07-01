List pagina's voor een tenant. Wordt gebruikt door de FChat desktopclient om de kamerlijst te vullen.  
Vereist dat `enableFChat` true is in de opgeloste aangepaste configuratie voor elke pagina.  
Pagina's die SSO vereisen worden gefilterd op basis van de groepsrechten van de verzoekende gebruiker.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nee | Onduidelijke pagineringscursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gekoppeld aan dezelfde `sortBy`. |
| limit | integer | query | Nee | 1..200, standaard 50 |
| q | string | query | Nee | Optionele hoofdletterongevoelige titelvoorvoegselfilter. |
| sortBy | string | query | Nee | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | Nee | Indien true, alleen pagina's teruggeven met ten minste één reactie. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Onduidelijke pagineringscursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gekoppeld aan dezelfde `sortBy`. (optioneel)
let limit = 987 // Int | 1..200, standaard 50 (optioneel)
let q = "q_example" // String | Optionele hoofdletterongevoelige titelvoorvoegselfilter. (optioneel)
let sortBy = PagesSortBy() // PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch). (optioneel)
let hasComments = true // Bool | Indien true, alleen pagina's teruggeven met ten minste één reactie. (optioneel)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]