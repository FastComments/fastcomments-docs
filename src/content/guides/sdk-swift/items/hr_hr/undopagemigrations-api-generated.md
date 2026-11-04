Postavlja batch posao koji poništava svaku migraciju iz jednog migrate-pages posla, od najnovije prema najstarijoj. Dostupno 90 dana.
Zapisi promijenjeni od migracije ostaju netaknuti, tako da kasnija izmjena nikada nije prebrisana.
Ispitujte GET /api/v1/batch-jobs/{batchJobId} za broj vraćenih/preskočenih zapisa.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| undoId | string | path | Da |  |

## Odgovor

Vraća: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Primjer

[inline-code-attrs-start title = 'undoPageMigrations primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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