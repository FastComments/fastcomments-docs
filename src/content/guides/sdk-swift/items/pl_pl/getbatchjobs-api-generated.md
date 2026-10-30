The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Only jobs of this type, for example migrate-pages. |
| status | string | query | No | Only jobs in this status: requested, running, success or failed. |
| skip | integer | query | No | How many jobs to skip, for paging. |

## Response

Zwraca: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Only jobs of this type, for example migrate-pages. (optional)
let status = "status_example" // String | Only jobs in this status: requested, running, success or failed. (optional)
let skip = 987 // Int | How many jobs to skip, for paging. (optional)

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