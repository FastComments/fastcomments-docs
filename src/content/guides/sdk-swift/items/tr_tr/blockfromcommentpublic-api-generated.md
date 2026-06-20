## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Örnek

[inline-code-attrs-start title = 'blockFromCommentPublic Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let publicBlockFromCommentParams = PublicBlockFromCommentParams(commentIds: ["commentIds_example"]) // PublicBlockFromCommentParams | 
let sso = "sso_example" // String |  (isteğe bağlı)

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

---