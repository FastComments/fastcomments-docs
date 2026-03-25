## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| userId | string | query | Ні |  |

## Відповідь

Повертає: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicket200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getTicket'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. У разі проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let userId = "userId_example" // String |  (необов'язково)

DefaultAPI.getTicket(tenantId: tenantId, id: id, userId: userId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---