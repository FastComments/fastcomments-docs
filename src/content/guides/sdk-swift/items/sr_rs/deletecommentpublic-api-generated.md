## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Да |  |
| editKey | string | query | Не |  |
| sso | string | query | Не |  |

## Response

Враћа: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIDeleteCommentResponse.swift)

## Пример

[inline-code-attrs-start title = 'deleteCommentPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let editKey = "editKey_example" // String |  (опционално)
let sso = "sso_example" // String |  (опционално)

PublicAPI.deleteCommentPublic(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, options: PublicAPI.DeleteCommentPublicOptions(editKey: editKey, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]