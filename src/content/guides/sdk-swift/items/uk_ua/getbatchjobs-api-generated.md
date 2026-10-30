Пакетні завдання орендаря (імпорт, експорт, міграція сторінок...), новіші спочатку.

## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| type | string | query | Ні | Лише завдання цього типу, наприклад migrate-pages. |
| status | string | query | Ні | Лише завдання у цьому статусі: requested, running, success або failed. |
| skip | integer | query | Ні | Скільки завдань пропустити, для пагінації. |

## Відповідь

Returns: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще у бета-версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Лише завдання цього типу, наприклад migrate-pages. (необов’язково)
let status = "status_example" // String | Лише завдання у цьому статусі: requested, running, success або failed. (необов’язково)
let skip = 987 // Int | Скільки завдань пропустити, для пагінації. (необов’язково)

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