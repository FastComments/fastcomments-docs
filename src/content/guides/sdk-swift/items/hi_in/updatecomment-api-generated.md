## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| contextUserId | string | query | नहीं |  |
| doSpamCheck | boolean | query | नहीं |  |
| isLive | boolean | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'updateComment उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updatableCommentParams = UpdatableCommentParams(urlId: "urlId_example", urlIdRaw: "urlIdRaw_example", url: "url_example", pageTitle: "pageTitle_example", userId: "userId_example", commenterEmail: "commenterEmail_example", commenterName: "commenterName_example", commenterLink: "commenterLink_example", comment: "comment_example", commentHTML: "commentHTML_example", parentId: "parentId_example", date: 123, localDateString: "localDateString_example", localDateHours: 123, votes: 123, votesUp: 123, votesDown: 123, expireAt: Date(), verified: false, verifiedDate: Date(), notificationSentForParent: false, notificationSentForParentTenant: false, reviewed: false, externalId: "externalId_example", externalParentId: "externalParentId_example", avatarSrc: "avatarSrc_example", isSpam: false, approved: false, isDeleted: false, isDeletedUser: false, isByAdmin: false, isByModerator: false, isPinned: false, isLocked: false, flagCount: 123, displayLabel: "displayLabel_example", meta: APICommentBase_meta(wpUserId: "wpUserId_example", wpPostId: "wpPostId_example"), moderationGroupIds: ["moderationGroupIds_example"], feedbackIds: ["feedbackIds_example"]) // UpdatableCommentParams | 
let contextUserId = "contextUserId_example" // String |  (वैकल्पिक)
let doSpamCheck = true // Bool |  (वैकल्पिक)
let isLive = true // Bool |  (वैकल्पिक)

DefaultAPI.updateComment(tenantId: tenantId, id: id, updatableCommentParams: updatableCommentParams, options: DefaultAPI.UpdateCommentOptions(contextUserId: contextUserId, doSpamCheck: doSpamCheck, isLive: isLive)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]