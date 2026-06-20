## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| broadcastId | string | query | Nie |  |
| isLive | boolean | query | Nie |  |
| doSpamCheck | boolean | query | Nie |  |
| skipDupCheck | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateFeedPostsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład createFeedPost'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createFeedPostParams = CreateFeedPostParams(title: "title_example", contentHTML: "contentHTML_example", media: [FeedPostMediaItem(title: "title_example", linkUrl: "linkUrl_example", sizes: [FeedPostMediaItemAsset(w: 123, h: 123, src: "src_example")])], links: [FeedPostLink(text: "text_example", title: "title_example", description: "description_example", url: "url_example")], fromUserId: "fromUserId_example", fromUserDisplayName: "fromUserDisplayName_example", tags: ["tags_example"], meta: "TODO") // CreateFeedPostParams | 
let broadcastId = "broadcastId_example" // String |  (opcjonalne)
let isLive = true // Bool |  (opcjonalne)
let doSpamCheck = true // Bool |  (opcjonalne)
let skipDupCheck = true // Bool |  (opcjonalne)

DefaultAPI.createFeedPost(tenantId: tenantId, createFeedPostParams: createFeedPostParams, broadcastId: broadcastId, isLive: isLive, doSpamCheck: doSpamCheck, skipDupCheck: skipDupCheck) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]