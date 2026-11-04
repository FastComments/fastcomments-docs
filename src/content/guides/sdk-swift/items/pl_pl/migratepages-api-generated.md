Kolejkuje zadanie wsadowe, które przenosi komentarze, strony i powiązane rekordy z jednego urlId lub domeny do innego.
Migracje "urlId" dopasowują tylko dokładny urlId. Migracje "domain" wyszukują/zamieniają wartość w każdym urlId i url.
Odpytuj GET /api/v1/batch-jobs/{batchJobId}: po zakończeniu zawiera wyniki poszczególnych migracji oraz undoId.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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