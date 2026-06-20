## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateFeedPostResponse.swift)

## 範例

[inline-code-attrs-start title = 'createFeedPostPublic 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createFeedPostParams = CreateFeedPostParams(title: "title_example", contentHTML: "contentHTML_example", media: [FeedPostMediaItem(title: "title_example", linkUrl: "linkUrl_example", sizes: [FeedPostMediaItemAsset(w: 123, h: 123, src: "src_example")])], links: [FeedPostLink(text: "text_example", title: "title_example", description: "description_example", url: "url_example")], fromUserId: "fromUserId_example", fromUserDisplayName: "fromUserDisplayName_example", tags: ["tags_example"], meta: "TODO") // CreateFeedPostParams | 
let broadcastId = "broadcastId_example" // String |  (可選)
let sso = "sso_example" // String |  (可選)

PublicAPI.createFeedPostPublic(tenantId: tenantId, createFeedPostParams: createFeedPostParams, broadcastId: broadcastId, sso: sso) { (response, error) in
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