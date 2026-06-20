## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIChildCommentsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад postCommentsByIds'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще перебувають на стадії бета. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentsByIdsParams = CommentsByIdsParams(ids: ["ids_example"]) // CommentsByIdsParams | 
let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.postCommentsByIds(commentsByIdsParams: commentsByIdsParams, sso: sso) { (response, error) in
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