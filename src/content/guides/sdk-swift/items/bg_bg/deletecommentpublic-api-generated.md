## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Да |  |
| editKey | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIDeleteCommentResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за deleteCommentPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни фрагменти от код все още са в бета. За проблеми, моля съобщете чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let editKey = "editKey_example" // String |  (по избор)
let sso = "sso_example" // String |  (по избор)

PublicAPI.deleteCommentPublic(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, editKey: editKey, sso: sso) { (response, error) in
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