Stellt einen Batch-Job in die Warteschlange, der Kommentare, Seiten und zugehörige Datensätze von einer urlId oder Domain auf eine andere verschiebt.
"urlId"-Migrationen treffen nur die exakte urlId. "domain"-Migrationen suchen/ersetzen den Wert in jeder urlId und url.
Rufen Sie GET /api/v1/batch-jobs/{batchJobId} ab: Sobald der Vorgang abgeschlossen ist, enthält er die Ergebnisse pro Migration und eine undoId.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'migratePages Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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