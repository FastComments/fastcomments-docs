## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Odziv

Vrne: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UnblockSuccess.swift)

## Primer

[inline-code-attrs-start title = 'unBlockCommentPublic Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta. Za vse težave prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let publicBlockFromCommentParams = PublicBlockFromCommentParams(commentIds: ["commentIds_example"]) // PublicBlockFromCommentParams | 
let sso = "sso_example" // String |  (neobvezno)

PublicAPI.unBlockCommentPublic(tenantId: tenantId, commentId: commentId, publicBlockFromCommentParams: publicBlockFromCommentParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]