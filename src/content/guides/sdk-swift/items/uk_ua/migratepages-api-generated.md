Додає у чергу пакетну задачу, яка переміщує коментарі, сторінки та пов’язані записи з одного urlId або домену в інший.  
"urlId" міграції відповідають лише точному urlId. "domain" міграції виконують пошук/заміну значення у кожному urlId та url.  
Опитування GET /api/v1/batch-jobs/{batchJobId}: після завершення вона повертає результати по кожній міграції та undoId.

## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Відповідь

Повертає: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще у бета‑версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
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