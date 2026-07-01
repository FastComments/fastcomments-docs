## Parametreler

| Ad | Tür | Konum | Gereklilik | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| commentId | string | yol | Evet |  |
| approved | boolean | sorgu | Hayır |  |
| broadcastId | string | sorgu | Hayır |  |
| sso | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## Örnek

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin.
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let approved = true // Bool |  (optional)
let broadcastId = "broadcastId_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

ModerationAPI.postSetCommentApprovalStatus(tenantId: tenantId, commentId: commentId, options: ModerationAPI.PostSetCommentApprovalStatusOptions(approved: approved, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]