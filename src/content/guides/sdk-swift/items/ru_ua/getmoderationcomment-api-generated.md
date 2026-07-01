## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| includeEmail | boolean | query | No |  |
| includeIP | boolean | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getModerationComment Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще бета. Якщо виникли проблеми, будь ласка, повідомте за посиланням http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (необов'язково)
let includeIP = true // Bool |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.getModerationComment(tenantId: tenantId, commentId: commentId, options: ModerationAPI.GetModerationCommentOptions(includeEmail: includeEmail, includeIP: includeIP, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]