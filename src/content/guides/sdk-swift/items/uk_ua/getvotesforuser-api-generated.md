## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlId | string | query | Так |  |
| userId | string | query | Ні |  |
| anonUserId | string | query | Ні |  |

## Відповідь

Повертає: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetVotesForUserResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getVotesForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще у бета-версії. Якщо виникнуть проблеми, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userId = "userId_example" // String |  (необов'язково)
let anonUserId = "anonUserId_example" // String |  (необов'язково)

DefaultAPI.getVotesForUser(tenantId: tenantId, urlId: urlId, userId: userId, anonUserId: anonUserId) { (response, error) in
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