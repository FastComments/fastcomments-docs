## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Risposta

Restituisce: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTickets200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Gli esempi di codice seguenti sono ancora in versione beta. Per qualsiasi problema, si prega di segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opzionale)
let state = 987 // Double |  (opzionale)
let skip = 987 // Double |  (opzionale)
let limit = 987 // Double |  (opzionale)

DefaultAPI.getTickets(tenantId: tenantId, userId: userId, state: state, skip: skip, limit: limit) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---