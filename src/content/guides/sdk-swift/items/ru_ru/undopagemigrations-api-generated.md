Ставит в очередь пакетное задание, которое отменяет каждую миграцию из одного задания migrate-pages, начиная с самых новых. Доступно в течение 90 дней.
Записи, изменённые после миграции, не затрагиваются, поэтому последующее редактирование никогда не будет перезаписано.
Опрашивайте GET /api/v1/batch-jobs/{batchJobId} для получения счётчиков восстановленных/пропущенных записей.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| undoId | string | path | Да |  |

## Ответ

Возвращает: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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