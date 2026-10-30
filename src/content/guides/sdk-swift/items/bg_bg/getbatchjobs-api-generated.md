The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Само задачи от този тип, например migrate-pages. |
| status | string | query | No | Само задачи в този статус: requested, running, success или failed. |
| skip | integer | query | No | Колко задачи да се пропуснат, за странициране. |

## Отговор

Връща: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Само задачи от този тип, например migrate-pages. (по избор)
let status = "status_example" // String | Само задачи в този статус: requested, running, success или failed. (по избор)
let skip = 987 // Int | Колко задачи да се пропуснат, за странициране. (по избор)

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