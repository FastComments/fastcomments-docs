Створює пакетне завдання, яке скасовує всі міграції одного завдання migrate-pages, починаючи з найновіших. Доступне протягом 90 днів.
Записи, змінені після міграції, не змінюються, тому пізніше редагування ніколи не буде перезаписано.
Опитуйте GET /api/v1/batch-jobs/{batchJobId}, щоб отримати кількість відновлених/пропущених записів.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| undoId | string | path | Так |  |

## Відповідь

Повертає: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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