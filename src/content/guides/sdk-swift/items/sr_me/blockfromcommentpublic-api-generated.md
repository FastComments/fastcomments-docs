## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Пример

[inline-code-attrs-start title = 'Пример blockFromCommentPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведени примјери кода су још увијек у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let publicBlockFromCommentParams = PublicBlockFromCommentParams(commentIds: ["commentIds_example"]) // PublicBlockFromCommentParams | 
let sso = "sso_example" // String |  (опционо)

PublicAPI.blockFromCommentPublic(tenantId: tenantId, commentId: commentId, publicBlockFromCommentParams: publicBlockFromCommentParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]