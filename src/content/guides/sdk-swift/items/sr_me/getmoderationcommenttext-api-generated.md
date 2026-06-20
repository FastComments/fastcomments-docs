## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Одговор

Враћа: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentTextResponse.swift)

## Примјер

[inline-code-attrs-start title = 'getModerationCommentText Примјер'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бети. За било који проблем, пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (опционо)

ModerationAPI.getModerationCommentText(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]