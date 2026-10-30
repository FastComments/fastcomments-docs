Die Batch-Jobs des Mandanten (Importe, Exporte, Seitenmigrationen...), neueste zuerst.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Nur Jobs dieses Typs, zum Beispiel migrate-pages. |
| status | string | query | No | Nur Jobs mit diesem Status: requested, running, success oder failed. |
| skip | integer | query | No | Wie viele Jobs übersprungen werden sollen, für die Seitennavigation. |

## Response

Returns: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'getBatchJobs Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Nur Jobs dieses Typs, zum Beispiel migrate-pages. (optional)
let status = "status_example" // String | Nur Jobs mit diesem Status: requested, running, success oder failed. (optional)
let skip = 987 // Int | Wie viele Jobs übersprungen werden sollen, für die Seitennavigation. (optional)

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