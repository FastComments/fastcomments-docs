## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| broadcastId | string | query | 예 |  |
| editKey | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentText200Response.swift)

## 예제

[inline-code-attrs-start title = 'setCommentText 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let commentTextUpdateRequest = CommentTextUpdateRequest(comment: "comment_example", mentions: [CommentUserMentionInfo(id: "id_example", tag: "tag_example", rawTag: "rawTag_example", type: "type_example", sent: false)], hashTags: [CommentUserHashTagInfo(id: "id_example", tag: "tag_example", url: "url_example", retain: false)]) // CommentTextUpdateRequest | 
let editKey = "editKey_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.setCommentText(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, commentTextUpdateRequest: commentTextUpdateRequest, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]