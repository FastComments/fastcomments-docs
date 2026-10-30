Партијски послови закупца (увози, извози, миграције страница...), прво најновије.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Само послови овог типа, на пример migrate-pages. |
| status | string | query | No | Само послови у овом статусу: requested, running, success или failed. |
| skip | integer | query | No | Колико послова треба прескочити, за пагинацију. |

## Одговор

Враћа: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'getBatchJobs Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем, молимо пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Само послови овог типа, на пример migrate-pages. (опционално)
let status = "status_example" // String | Само послови у овом статусу: requested, running, success или failed. (опционално)
let skip = 987 // Int | Колико послова треба прескочити, за пагинацију. (опционално)

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