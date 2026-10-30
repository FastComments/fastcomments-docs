## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |

## Відповідь

Повертає: [`GetPollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще у бета-версії. У разі проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 

DefaultAPI.getPoll(tenantId: tenantId, commentId: commentId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]