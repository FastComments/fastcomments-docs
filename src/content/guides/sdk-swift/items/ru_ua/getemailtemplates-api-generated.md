## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Ответ

Возвращает: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplates200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример getEmailTemplates'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. Если возникнут проблемы, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (необязательно)

DefaultAPI.getEmailTemplates(tenantId: tenantId, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]