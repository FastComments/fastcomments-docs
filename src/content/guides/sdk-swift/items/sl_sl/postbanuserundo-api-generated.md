## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer postBanUserUndo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta. Za morebitne težave poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let banUserUndoParams = BanUserUndoParams(changelog: APIBanUserChangeLog(createdBannedUserId: "createdBannedUserId_example", updatedBannedUserId: "updatedBannedUserId_example", deletedBannedUsers: [APIBannedUser(id: "id_example", tenantId: "tenantId_example", userId: "userId_example", email: "email_example", username: "username_example", ipHash: "ipHash_example", createdAt: Date(), bannedByUserId: "bannedByUserId_example", bannedCommentText: "bannedCommentText_example", banType: "banType_example", bannedUntil: Date(), hasEmailWildcard: false, banReason: "banReason_example")], changedValuesBefore: APIBanUserChangedValues(id: "id_example", tenantId: "tenantId_example", userId: "userId_example", email: "email_example", username: "username_example", ipHash: "ipHash_example", createdAt: Date(), bannedByUserId: "bannedByUserId_example", bannedCommentText: "bannedCommentText_example", banType: "banType_example", bannedUntil: Date(), hasEmailWildcard: false, banReason: "banReason_example"))) // BanUserUndoParams | 
let sso = "sso_example" // String |  (izbirno)

ModerationAPI.postBanUserUndo(banUserUndoParams: banUserUndoParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]