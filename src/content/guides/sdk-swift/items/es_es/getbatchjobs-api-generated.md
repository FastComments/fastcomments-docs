The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Sólo trabajos de este tipo, por ejemplo migrate-pages. |
| status | string | query | No | Sólo trabajos con este estado: requested, running, success o failed. |
| skip | integer | query | No | Cuántos trabajos omitir, para paginación. |

## Response

Returns: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'Ejemplo getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código están todavía en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Sólo trabajos de este tipo, por ejemplo migrate-pages. (opcional)
let status = "status_example" // String | Sólo trabajos con este estado: requested, running, success o failed. (opcional)
let skip = 987 // Int | Cuántos trabajos omitir, para paginación. (opcional)

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