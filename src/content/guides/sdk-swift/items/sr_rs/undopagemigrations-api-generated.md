Стави у ред групни посао који поништава сваку миграцију из једног migrate-pages посла, од најновијег. Доступно 90 дана.
Записи промењени од миграције се не дирају, тако да каснија измена никада неће бити преписана.
Питај GET /api/v1/batch-jobs/{batchJobId} за бројеве враћених/прескочених.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| undoId | string | path | Да |  |

## Одговор

Враћа: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'undoPageMigrations пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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