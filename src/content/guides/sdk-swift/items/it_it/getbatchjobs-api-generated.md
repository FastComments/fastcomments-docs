The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| type | string | query | No | Solo i lavori di questo tipo, ad esempio migrate-pages. |
| status | string | query | No | Solo i lavori in questo stato: requested, running, success o failed. |
| skip | integer | query | No | Quanti lavori saltare, per paginazione. |

## Response

Restituisce: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'Esempio getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalalo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Solo i lavori di questo tipo, ad esempio migrate-pages. (opzionale)
let status = "status_example" // String | Solo i lavori in questo stato: requested, running, success o failed. (opzionale)
let skip = 987 // Int | Quanti lavori saltare, per paginazione. (opzionale)

DefaultAPI.getBatchJobs(tenantId: tenantId, options: DefaultAPI.GetBatchJobsOptions(type: type, status: status, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]