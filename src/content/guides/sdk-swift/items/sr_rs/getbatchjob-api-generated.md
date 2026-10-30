Статус, напредак и, када се заврши, резултат једног пакетног посла.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetBatchJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'getBatchJob Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем, молимо пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getBatchJob(tenantId: tenantId, id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]