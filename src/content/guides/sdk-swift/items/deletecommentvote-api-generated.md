## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| voteId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| broadcastId | string | query | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteCommentVote200Response.swift)

## Example

[inline-code-attrs-start title = 'deleteCommentVote Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voteId = "voteId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let editKey = "editKey_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.deleteCommentVote(tenantId: tenantId, commentId: commentId, voteId: voteId, urlId: urlId, broadcastId: broadcastId, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
