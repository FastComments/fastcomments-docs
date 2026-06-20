Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la sua lista delle stanze.
Richiede che `enableFChat` sia true nella custom config risolta per ogni pagina.
Le pagine che richiedono SSO vengono filtrate in base all'accesso dei gruppi dell'utente richiedente.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale per prefisso del titolo, insensibile alle maiuscole/minuscole. |
| sortBy | string | query | No | Ordine di ordinamento. `updatedAt` (predefinito, dal più recente), `commentCount` (prima le pagine con più commenti), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, ritorna solo le pagine con almeno un commento. |

## Response

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per eventuali problemi, segnalarli tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. (opzionale)
let limit = 987 // Int | 1..200, predefinito 50 (opzionale)
let q = "q_example" // String | Filtro opzionale per prefisso del titolo, insensibile alle maiuscole/minuscole. (opzionale)
let sortBy = PagesSortBy() // PagesSortBy | Ordine di ordinamento. `updatedAt` (predefinito, dal più recente), `commentCount` (prima le pagine con più commenti), o `title` (alfabetico). (opzionale)
let hasComments = true // Bool | Se true, ritorna solo le pagine con almeno un commento. (opzionale)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]