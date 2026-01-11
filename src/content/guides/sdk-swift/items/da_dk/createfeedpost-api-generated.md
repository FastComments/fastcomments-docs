## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| broadcastId | string | query | Nej |  |
| isLive | boolean | query | Nej |  |
| doSpamCheck | boolean | query | Nej |  |
| skipDupCheck | boolean | query | Nej |  |

## Svar

Returnerer: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateFeedPost200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'createFeedPost Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis der opstår et problem, indberet venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createFeedPostParams = CreateFeedPostParams(title: "title_example", contentHTML: "contentHTML_example", media: [FeedPostMediaItem(title: "title_example", linkUrl: "linkUrl_example", sizes: [FeedPostMediaItemAsset(w: 123, h: 123, src: "src_example")])], links: [FeedPostLink(text: "text_example", title: "title_example", description: "description_example", url: "url_example")], fromUserId: "fromUserId_example", fromUserDisplayName: "fromUserDisplayName_example", tags: ["tags_example"], meta: "TODO") // CreateFeedPostParams | 
let broadcastId = "broadcastId_example" // String |  (valgfri)
let isLive = true // Bool |  (valgfri)
let doSpamCheck = true // Bool |  (valgfri)
let skipDupCheck = true // Bool |  (valgfri)

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