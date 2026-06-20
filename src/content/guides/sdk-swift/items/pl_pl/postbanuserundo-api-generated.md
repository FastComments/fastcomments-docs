## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład postBanUserUndo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let banUserUndoParams = BanUserUndoParams(changelog: APIBanUserChangeLog(createdBannedUserId: "createdBannedUserId_example", updatedBannedUserId: "updatedBannedUserId_example", deletedBannedUsers: [APIBannedUser(id: "id_example", tenantId: "tenantId_example", userId: "userId_example", email: "email_example", username: "username_example", ipHash: "ipHash_example", createdAt: Date(), bannedByUserId: "bannedByUserId_example", bannedCommentText: "bannedCommentText_example", banType: "banType_example", bannedUntil: Date(), hasEmailWildcard: false, banReason: "banReason_example")], changedValuesBefore: APIBanUserChangedValues(id: "id_example", tenantId: "tenantId_example", userId: "userId_example", email: "email_example", username: "username_example", ipHash: "ipHash_example", createdAt: Date(), bannedByUserId: "bannedByUserId_example", bannedCommentText: "bannedCommentText_example", banType: "banType_example", bannedUntil: Date(), hasEmailWildcard: false, banReason: "banReason_example"))) // BanUserUndoParams | 
let sso = "sso_example" // String | (opcjonalne)

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