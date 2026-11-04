Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la sua lista di stanze.
Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina.
Le pagine che richiedono SSO vengono filtrate in base all'accesso ai gruppi dell'utente richiedente.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale per prefisso del titolo, senza distinzione tra maiuscole e minuscole. |
| sortBy | string | query | No | Ordinamento. `updatedAt` (predefinito, dal più recente), `commentCount` (dal maggior numero di commenti), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. (optional)
let limit = 987 // Int | 1..200, predefinito 50 (optional)
let q = "q_example" // String | Filtro opzionale per prefisso del titolo, senza distinzione tra maiuscole e minuscole. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Ordinamento. `updatedAt` (predefinito, dal più recente), `commentCount` (dal maggior numero di commenti), o `title` (alfabetico). (optional)
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