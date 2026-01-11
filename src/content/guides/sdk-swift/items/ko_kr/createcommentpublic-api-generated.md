---
## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| broadcastId | string | query | 예 |  |
| sessionId | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateCommentPublic200Response.swift)

## 예제

[inline-code-attrs-start title = 'createCommentPublic 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let commentData = CommentData(date: 123, localDateString: "localDateString_example", localDateHours: 123, commenterName: "commenterName_example", commenterEmail: "commenterEmail_example", commenterLink: "commenterLink_example", comment: "comment_example", productId: 123, userId: "userId_example", avatarSrc: "avatarSrc_example", parentId: "parentId_example", mentions: [CommentUserMentionInfo(id: "id_example", tag: "tag_example", rawTag: "rawTag_example", type: "type_example", sent: false)], hashTags: [CommentUserHashTagInfo(id: "id_example", tag: "tag_example", url: "url_example", retain: false)], pageTitle: "pageTitle_example", isFromMyAccountPage: false, url: "url_example", urlId: "urlId_example", meta: 123, moderationGroupIds: ["moderationGroupIds_example"], rating: 123, fromOfflineRestore: false, autoplayDelayMS: 123, feedbackIds: ["feedbackIds_example"], questionValues: "TODO") // CommentData | 
let sessionId = "sessionId_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

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

---