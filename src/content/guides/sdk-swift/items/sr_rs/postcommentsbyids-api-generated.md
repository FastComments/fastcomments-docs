## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIChildCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'postCommentsByIds Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи код примери су још у бета верзији. За сваки проблем, молимо пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentsByIdsParams = CommentsByIdsParams(ids: ["ids_example"]) // CommentsByIdsParams | 
let sso = "sso_example" // String |  (опционално)

ModerationAPI.postCommentsByIds(tenantId: tenantId, commentsByIdsParams: commentsByIdsParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]