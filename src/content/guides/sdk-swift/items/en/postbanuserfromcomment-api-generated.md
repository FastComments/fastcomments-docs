## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Example

[inline-code-attrs-start title = 'postBanUserFromComment Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (optional)
let banEmailDomain = true // Bool |  (optional)
let banIP = true // Bool |  (optional)
let deleteAllUsersComments = true // Bool |  (optional)
let bannedUntil = "bannedUntil_example" // String |  (optional)
let isShadowBan = true // Bool |  (optional)
let updateId = "updateId_example" // String |  (optional)
let banReason = "banReason_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

ModerationAPI.postBanUserFromComment(commentId: commentId, banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
