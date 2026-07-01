## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Відповідь

Повертає: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteCommentResult.swift)

## Приклад

[inline-code-attrs-start title = 'deleteComment Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду ще у бета‑версії. У разі будь‑якої проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let contextUserId = "contextUserId_example" // String |  (необов'язковий)
let isLive = true // Bool |  (необов'язковий)

DefaultAPI.deleteComment(tenantId: tenantId, id: id, options: DefaultAPI.DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive)) { (response, error) in
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