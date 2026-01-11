## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| contextUserId | string | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| isLive | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Örnek

[inline-code-attrs-start title = 'updateComment Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updatableCommentParams = UpdatableCommentParams(urlId: "urlId_example", urlIdRaw: "urlIdRaw_example", url: "url_example", pageTitle: "pageTitle_example", userId: "userId_example", commenterEmail: "commenterEmail_example", commenterName: "commenterName_example", commenterLink: "commenterLink_example", comment: "comment_example", commentHTML: "commentHTML_example", parentId: "parentId_example", date: 123, localDateString: "localDateString_example", localDateHours: 123, votes: 123, votesUp: 123, votesDown: 123, expireAt: Date(), verified: false, verifiedDate: Date(), notificationSentForParent: false, notificationSentForParentTenant: false, reviewed: false, externalId: "externalId_example", externalParentId: "externalParentId_example", avatarSrc: "avatarSrc_example", isSpam: false, approved: false, isDeleted: false, isDeletedUser: false, isByAdmin: false, isByModerator: false, isPinned: false, isLocked: false, flagCount: 123, displayLabel: "displayLabel_example", meta: FComment_meta(wpUserId: "wpUserId_example", wpPostId: "wpPostId_example"), moderationGroupIds: ["moderationGroupIds_example"], feedbackIds: ["feedbackIds_example"]) // UpdatableCommentParams | 
let contextUserId = "contextUserId_example" // String |  (isteğe bağlı)
let doSpamCheck = true // Bool |  (isteğe bağlı)
let isLive = true // Bool |  (isteğe bağlı)

DefaultAPI.updateComment(tenantId: tenantId, id: id, updatableCommentParams: updatableCommentParams, contextUserId: contextUserId, doSpamCheck: doSpamCheck, isLive: isLive) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]