## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## תגובה

מחזיר: [`[SaveCommentsBulkResponse]`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/[SaveCommentsBulkResponse].swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת saveCommentsBulk'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הקוד הבא עדיין בגירסת בטא. לכל בעיה, אנא דווח בכתובת http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createCommentParams = [CreateCommentParams(date: 123, localDateString: "localDateString_example", localDateHours: 123, commenterName: "commenterName_example", commenterEmail: "commenterEmail_example", commenterLink: "commenterLink_example", comment: "comment_example", productId: 123, userId: "userId_example", avatarSrc: "avatarSrc_example", parentId: "parentId_example", mentions: [CommentUserMentionInfo(id: "id_example", tag: "tag_example", rawTag: "rawTag_example", type: "type_example", sent: false)], hashTags: [CommentUserHashTagInfo(id: "id_example", tag: "tag_example", url: "url_example", retain: false)], pageTitle: "pageTitle_example", isFromMyAccountPage: false, url: "url_example", urlId: "urlId_example", meta: 123, moderationGroupIds: ["moderationGroupIds_example"], rating: 123, fromOfflineRestore: false, autoplayDelayMS: 123, feedbackIds: ["feedbackIds_example"], questionValues: "TODO", tos: false, botId: "botId_example", approved: false, domain: "domain_example", ip: "ip_example", isPinned: false, locale: "locale_example", reviewed: false, verified: false, votes: 123, votesDown: 123, votesUp: 123)] // [CreateCommentParams] | 
let isLive = true // Bool |  (אופציונלי)
let doSpamCheck = true // Bool |  (אופציונלי)
let sendEmails = true // Bool |  (אופציונלי)
let populateNotifications = true // Bool |  (אופציונלי)

DefaultAPI.saveCommentsBulk(tenantId: tenantId, createCommentParams: createCommentParams, options: DefaultAPI.SaveCommentsBulkOptions(isLive: isLive, doSpamCheck: doSpamCheck, sendEmails: sendEmails, populateNotifications: populateNotifications)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]