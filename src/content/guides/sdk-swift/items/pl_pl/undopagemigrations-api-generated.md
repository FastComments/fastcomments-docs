Kolejkuje zadanie wsadowe, które odwraca każdą migrację z jednego zadania migrate-pages, od najnowszych. Dostępne przez 90 dni.
Rekordy zmienione od czasu migracji pozostają nietknięte, więc późniejsza edycja nigdy nie zostanie nadpisana.
Odpytuj GET /api/v1/batch-jobs/{batchJobId}, aby uzyskać liczbę przywróconych/pominiętych rekordów.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| undoId | string | path | Tak |  |

## Odpowiedź

Zwraca: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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