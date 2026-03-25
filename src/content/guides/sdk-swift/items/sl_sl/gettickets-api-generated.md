## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| state | number | query | Ne |  |
| skip | number | query | Ne |  |
| limit | number | query | Ne |  |

## Odgovor

Vrača: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTickets200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za kakršno koli težavo, prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (neobvezno)
let state = 987 // Double |  (neobvezno)
let skip = 987 // Double |  (neobvezno)
let limit = 987 // Double |  (neobvezno)

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