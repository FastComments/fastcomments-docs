---
Stellt einen Batch‑Job in die Warteschlange, der jede Migration von einem *migrate‑pages*‑Job umkehrt, beginnend mit dem neuesten. Verfügbar für 90 Tage. Änderungen, die seit der Migration vorgenommen wurden, bleiben unverändert, sodass eine spätere Bearbeitung niemals überschrieben wird. Abfragen Sie **GET** `/api/v1/batch-jobs/{batchJobId}` für die wiederhergestellten/übersprungenen Zähler.

## Parameter

| Name     | Typ    | Ort   | Erforderlich | Beschreibung |
|----------|--------|-------|--------------|--------------|
| tenantId | string | query | Ja           |  |
| undoId   | string | path  | Ja           |  |

## Antwort

Rückgabe: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'undoPageMigrations Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta‑Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
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

---