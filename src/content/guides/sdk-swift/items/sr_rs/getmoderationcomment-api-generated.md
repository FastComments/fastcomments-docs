---
## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| includeEmail | boolean | query | Не |  |
| includeIP | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getModerationComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примерi кода су још у бета фази. За било какав проблем пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (опционо)
let includeIP = true // Bool |  (опционо)
let sso = "sso_example" // String |  (опционо)

ModerationAPI.getModerationComment(commentId: commentId, includeEmail: includeEmail, includeIP: includeIP, sso: sso) { (response, error) in
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