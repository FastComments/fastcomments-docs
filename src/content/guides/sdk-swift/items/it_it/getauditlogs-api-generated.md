## Parametri

| Name | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Risposta

Restituisce: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalalo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (opzionale)
let skip = 987 // Double |  (opzionale)
let order = SORT_DIR() // SORTDIR |  (opzionale)
let after = 987 // Double |  (opzionale)
let before = 987 // Double |  (opzionale)

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