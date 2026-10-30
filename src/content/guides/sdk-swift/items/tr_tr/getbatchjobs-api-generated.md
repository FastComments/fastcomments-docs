The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| type | string | query | Hayır | Bu türdeki yalnızca işler, örneğin migrate-pages. |
| status | string | query | Hayır | Bu durumda olan yalnızca işler: requested, running, success veya failed. |
| skip | integer | query | Hayır | Sayfalama için atlanacak iş sayısı. |

## Response

Döndürür: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getBatchJobs Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta sürümündedir. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Bu türdeki yalnızca işler, örneğin migrate-pages. (isteğe bağlı)
let status = "status_example" // String | Bu durumda olan yalnızca işler: requested, running, success veya failed. (isteğe bağlı)
let skip = 987 // Int | Sayfalama için atlanacak iş sayısı. (isteğe bağlı)

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