## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| postId | string | path | 예 |  |
| broadcastId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateFeedPostResponse.swift)

## 예제

[inline-code-attrs-start title = 'updateFeedPostPublic 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postId = "postId_example" // String | 
let updateFeedPostParams = UpdateFeedPostParams(title: "title_example", contentHTML: "contentHTML_example", media: [FeedPostMediaItem(title: "title_example", linkUrl: "linkUrl_example", sizes: [FeedPostMediaItemAsset(w: 123, h: 123, src: "src_example")])], links: [FeedPostLink(text: "text_example", title: "title_example", description: "description_example", url: "url_example")], tags: ["tags_example"], meta: "TODO") // UpdateFeedPostParams | 
let broadcastId = "broadcastId_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.updateFeedPostPublic(tenantId: tenantId, postId: postId, updateFeedPostParams: updateFeedPostParams, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]