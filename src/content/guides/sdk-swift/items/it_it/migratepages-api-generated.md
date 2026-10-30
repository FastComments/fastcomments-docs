Accoda un lavoro batch che sposta commenti, pagine e record correlati da un urlId o dominio a un altro.  
\"urlId\" migrazioni corrispondono solo all'esatto urlId. \"domain\" migrazioni trovano/sostituiscono il valore all'interno di ogni urlId e url.  
Interroga GET /api/v1/batch-jobs/{batchJobId}: una volta completato, restituisce i risultati per migrazione e un undoId.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Risposta

Restituisce: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let migrateAPIPagesData = MigrateAPIPagesData(migrations: [PageMigration(from: "from_example", to: "to_example", type: PageMigrationType())]) // MigrateAPIPagesData | 

DefaultAPI.migratePages(tenantId: tenantId, migrateAPIPagesData: migrateAPIPagesData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---