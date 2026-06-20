## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateFeedPostResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад createFeedPostPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще в бета-версії. Якщо виникнуть проблеми, будь ласка, повідомте їх через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createFeedPostParams = CreateFeedPostParams(title: "title_example", contentHTML: "contentHTML_example", media: [FeedPostMediaItem(title: "title_example", linkUrl: "linkUrl_example", sizes: [FeedPostMediaItemAsset(w: 123, h: 123, src: "src_example")])], links: [FeedPostLink(text: "text_example", title: "title_example", description: "description_example", url: "url_example")], fromUserId: "fromUserId_example", fromUserDisplayName: "fromUserDisplayName_example", tags: ["tags_example"], meta: "TODO") // CreateFeedPostParams | 
let broadcastId = "broadcastId_example" // String |  (необов'язковий)
let sso = "sso_example" // String |  (необов'язковий)

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