The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Uniquement les travaux de ce type, par exemple migrate-pages. |
| status | string | query | No | Uniquement les travaux dans cet état : requested, running, success ou failed. |
| skip | integer | query | No | Nombre de travaux à ignorer, pour la pagination. |

## Response

Renvoie: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemple getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Uniquement les travaux de ce type, par exemple migrate-pages. (optionnel)
let status = "status_example" // String | Uniquement les travaux dans cet état : requested, running, success ou failed. (optionnel)
let skip = 987 // Int | Nombre de travaux à ignorer, pour la pagination. (optionnel)

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