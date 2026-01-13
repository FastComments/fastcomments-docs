## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| redirectURL | string | query | Ні |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад sendLoginLink'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще перебувають у бета-версії. Якщо виникнуть проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let redirectURL = "redirectURL_example" // String |  (необов'язково)

DefaultAPI.sendLoginLink(tenantId: tenantId, id: id, redirectURL: redirectURL) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]