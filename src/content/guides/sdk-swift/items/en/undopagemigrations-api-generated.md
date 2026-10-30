Queues a batch job that reverses every migration from one migrate-pages job, newest first. Available for 90 days.
Records changed since the migration are left alone, so a later edit is never overwritten.
Poll GET /api/v1/batch-jobs/{batchJobId} for the restored/skipped counts.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| undoId | string | path | Yes |  |

## Response

Returns: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'undoPageMigrations Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let undoId = "undoId_example" // String | 

DefaultAPI.undoPageMigrations(tenantId: tenantId, undoId: undoId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
