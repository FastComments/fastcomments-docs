## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Respons

Retourneert: [`[SaveComment200Response]`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/[SaveComment200Response].swift)

## Voorbeeld

[inline-code-attrs-start title = 'saveCommentsBulk Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, meld ze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createCommentParams = [CreateCommentParams(date: 123, localDateString: "localDateString_example", localDateHours: 123, commenterName: "commenterName_example", commenterEmail: "commenterEmail_example", commenterLink: "commenterLink_example", comment: "comment_example", productId: 123, userId: "userId_example", avatarSrc: "avatarSrc_example", parentId: "parentId_example", mentions: [CommentUserMentionInfo(id: "id_example", tag: "tag_example", rawTag: "rawTag_example", type: "type_example", sent: false)], hashTags: [CommentUserHashTagInfo(id: "id_example", tag: "tag_example", url: "url_example", retain: false)], pageTitle: "pageTitle_example", isFromMyAccountPage: false, url: "url_example", urlId: "urlId_example", meta: 123, moderationGroupIds: ["moderationGroupIds_example"], rating: 123, fromOfflineRestore: false, autoplayDelayMS: 123, feedbackIds: ["feedbackIds_example"], questionValues: "TODO", approved: false, domain: "domain_example", ip: "ip_example", isPinned: false, locale: "locale_example", reviewed: false, verified: false, votes: 123, votesDown: 123, votesUp: 123)] // [CreateCommentParams] | 
let isLive = true // Bool |  (optioneel)
let doSpamCheck = true // Bool |  (optioneel)
let sendEmails = true // Bool |  (optioneel)
let populateNotifications = true // Bool |  (optioneel)

DefaultAPI.saveCommentsBulk(tenantId: tenantId, createCommentParams: createCommentParams, isLive: isLive, doSpamCheck: doSpamCheck, sendEmails: sendEmails, populateNotifications: populateNotifications) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]