Пакетные задания арендатора (импорт, экспорт, миграция страниц...), от новых к старым.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| type | string | query | Нет | Только задания этого типа, например migrate-pages. |
| status | string | query | Нет | Только задания со статусом: requested, running, success или failed. |
| skip | integer | query | Нет | Сколько заданий пропустить, для постраничного вывода. |

## Ответ

Возвращает: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Только задания этого типа, например migrate-pages. (optional)
let status = "status_example" // String | Только задания со статусом: requested, running, success или failed. (optional)
let skip = 987 // Int | Сколько заданий пропустить, для постраничного вывода. (optional)

DefaultAPI.getBatchJobs(tenantId: tenantId, options: DefaultAPI.GetBatchJobsOptions(type: type, status: status, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]