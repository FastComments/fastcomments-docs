Queues a batch job that reverses every migration from one migrate-pages job, newest first. Beschikbaar gedurende 90 dagen.  
Records changed since the migration are left alone, so a later edit is never overwritten. Records die sinds de migratie zijn gewijzigd, blijven onaangeroerd, zodat een latere bewerking nooit wordt overschreven.  
Poll GET /api/v1/batch-jobs/{batchJobId} for the restored/skipped counts. Poll GET /api/v1/batch-jobs/{batchJobId} voor de herstelde/overgeslagen tellingen.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| undoId | string | path | Ja |  |

## Response

Retourneert: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'undoPageMigrations Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
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