## Paramètres

| Name | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| contextUserId | string | query | Non |  |
| doSpamCheck | boolean | query | Non |  |
| isLive | boolean | query | Non |  |

## Réponse

Retourne: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updatableCommentParams = UpdatableCommentParams(urlId: "urlId_example", urlIdRaw: "urlIdRaw_example", url: "url_example", pageTitle: "pageTitle_example", userId: "userId_example", commenterEmail: "commenterEmail_example", commenterName: "commenterName_example", commenterLink: "commenterLink_example", comment: "comment_example", commentHTML: "commentHTML_example", parentId: "parentId_example", date: 123, localDateString: "localDateString_example", localDateHours: 123, votes: 123, votesUp: 123, votesDown: 123, expireAt: Date(), verified: false, verifiedDate: Date(), notificationSentForParent: false, notificationSentForParentTenant: false, reviewed: false, externalId: "externalId_example", externalParentId: "externalParentId_example", avatarSrc: "avatarSrc_example", isSpam: false, approved: false, isDeleted: false, isDeletedUser: false, isByAdmin: false, isByModerator: false, isPinned: false, isLocked: false, flagCount: 123, displayLabel: "displayLabel_example", meta: FComment_meta(wpUserId: "wpUserId_example", wpPostId: "wpPostId_example"), moderationGroupIds: ["moderationGroupIds_example"], feedbackIds: ["feedbackIds_example"]) // UpdatableCommentParams | 
let contextUserId = "contextUserId_example" // String |  (optionnel)
let doSpamCheck = true // Bool |  (optionnel)
let isLive = true // Bool |  (optionnel)

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

---