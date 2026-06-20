## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| commentId | string | path | Evet |  |
| editKey | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIGetCommentTextResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getCommentText Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let editKey = "editKey_example" // String |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)

PublicAPI.getCommentText(tenantId: tenantId, commentId: commentId, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]