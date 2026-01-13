## Параметры

| Имя | Тип | Местоположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantUsers200Response.swift)

## Пример

[inline-code-attrs-start title = 'getTenantUsers Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. При возникновении проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (необязательно)

DefaultAPI.getTenantUsers(tenantId: tenantId, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]