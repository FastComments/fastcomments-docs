## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| limit | number | query | Nie |  |
| skip | number | query | Nie |  |
| order | string | query | Nie |  |
| after | number | query | Nie |  |
| before | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogs200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemów prosimy zgłaszać na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (opcjonalne)
let skip = 987 // Double |  (opcjonalne)
let order = SORT_DIR() // SORTDIR |  (opcjonalne)
let after = 987 // Double |  (opcjonalne)
let before = 987 // Double |  (opcjonalne)

DefaultAPI.getAuditLogs(tenantId: tenantId, limit: limit, skip: skip, order: order, after: after, before: before) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]