Accoda un job batch che annulla ogni migrazione di un job migrate-pages, dal più recente al più vecchio. Disponibile per 90 giorni.
I record modificati dopo la migrazione vengono lasciati invariati, quindi una modifica successiva non viene mai sovrascritta.
Interroga GET /api/v1/batch-jobs/{batchJobId} per i conteggi ripristinati/saltati.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| undoId | string | path | Sì |  |

## Risposta

Restituisce: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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