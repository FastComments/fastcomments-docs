## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | zapytanie | Tak |  |
| userId | string | zapytanie | Nie |  |
| state | number | zapytanie | Nie |  |
| skip | number | zapytanie | Nie |  |
| limit | number | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemu zgłoś to przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcjonalne)
let state = 987 // Double |  (opcjonalne)
let skip = 987 // Double |  (opcjonalne)
let limit = 987 // Double |  (opcjonalne)

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