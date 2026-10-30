Идентифицира използваните идентификационни данни: наема, към който принадлежи, и за OAuth токени – потребителя, който е упълномощил това. Интеграциите използват това за тестване на връзка и етикетиране.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Пример

[inline-code-attrs-start title = 'getMe Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са бета. За каквито и да е проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]