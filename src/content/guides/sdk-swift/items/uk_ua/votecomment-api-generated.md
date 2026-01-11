## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| urlId | string | query | Так |  |
| broadcastId | string | query | Так |  |
| sessionId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteComment200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад voteComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. Для будь-якої проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let voteBodyParams = VoteBodyParams(commenterEmail: "commenterEmail_example", commenterName: "commenterName_example", voteDir: "voteDir_example", url: "url_example") // VoteBodyParams | 
let sessionId = "sessionId_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

PublicAPI.voteComment(tenantId: tenantId, commentId: commentId, urlId: urlId, broadcastId: broadcastId, voteBodyParams: voteBodyParams, sessionId: sessionId, sso: sso) { (response, error) in
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