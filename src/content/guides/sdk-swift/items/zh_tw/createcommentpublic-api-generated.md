## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| urlId | string | 查詢 | 是 |  |
| broadcastId | string | 查詢 | 是 |  |
| sessionId | string | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SaveCommentsResponseWithPresence.swift)

## 範例

[inline-code-attrs-start title = 'createCommentPublic 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let commentData = CommentData(date: 123, localDateString: "localDateString_example", localDateHours: 123, commenterName: "commenterName_example", commenterEmail: "commenterEmail_example", commenterLink: "commenterLink_example", comment: "comment_example", productId: 123, userId: "userId_example", avatarSrc: "avatarSrc_example", parentId: "parentId_example", mentions: [CommentUserMentionInfo(id: "id_example", tag: "tag_example", rawTag: "rawTag_example", type: "type_example", sent: false)], hashTags: [CommentUserHashTagInfo(id: "id_example", tag: "tag_example", url: "url_example", retain: false)], pageTitle: "pageTitle_example", isFromMyAccountPage: false, url: "url_example", urlId: "urlId_example", meta: 123, moderationGroupIds: ["moderationGroupIds_example"], rating: 123, fromOfflineRestore: false, autoplayDelayMS: 123, feedbackIds: ["feedbackIds_example"], questionValues: "TODO", tos: false, botId: "botId_example") // CommentData | 
let sessionId = "sessionId_example" // String |  (可選)
let sso = "sso_example" // String |  (可選)

PublicAPI.createCommentPublic(tenantId: tenantId, urlId: urlId, broadcastId: broadcastId, commentData: commentData, sessionId: sessionId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]