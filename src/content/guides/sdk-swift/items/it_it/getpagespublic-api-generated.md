List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale del prefisso del titolo, insensibile al maiuscolo/minuscolo. |
| sortBy | string | query | No | Ordine di ordinamento. `updatedAt` (predefinito, più recenti per primi), `commentCount` (più commenti per primi), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in fase beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. (optional)
let limit = 987 // Int | 1..200, predefinito 50 (optional)
let q = "q_example" // String | Filtro opzionale del prefisso del titolo, insensibile al maiuscolo/minuscolo. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Ordine di ordinamento. `updatedAt` (predefinito, più recenti per primi), `commentCount` (più commenti per primi), o `title` (alfabetico). (optional)
let hasComments = true // Bool | Se true, restituisce solo le pagine con almeno un commento. (optional)

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