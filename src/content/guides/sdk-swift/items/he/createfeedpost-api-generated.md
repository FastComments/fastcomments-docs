## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| broadcastId | string | query | לא |  |
| isLive | boolean | query | לא |  |
| doSpamCheck | boolean | query | לא |  |
| skipDupCheck | boolean | query | לא |  |

## תגובה

מחזיר: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateFeedPostsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createFeedPost'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בגרסת בטא. עבור כל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createFeedPostParams = CreateFeedPostParams(title: "title_example", contentHTML: "contentHTML_example", media: [FeedPostMediaItem(title: "title_example", linkUrl: "linkUrl_example", sizes: [FeedPostMediaItemAsset(w: 123, h: 123, src: "src_example")])], links: [FeedPostLink(text: "text_example", title: "title_example", description: "description_example", url: "url_example")], fromUserId: "fromUserId_example", fromUserDisplayName: "fromUserDisplayName_example", tags: ["tags_example"], meta: "TODO") // CreateFeedPostParams | 
let broadcastId = "broadcastId_example" // String |  (אופציונלי)
let isLive = true // Bool |  (אופציונלי)
let doSpamCheck = true // Bool |  (אופציונלי)
let skipDupCheck = true // Bool |  (אופציונלי)

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