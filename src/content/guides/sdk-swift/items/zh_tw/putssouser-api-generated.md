## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| updateComments | boolean | query | 否 |  |

## 回應

傳回: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PutSSOUserAPIResponse.swift)

## 範例

[inline-code-attrs-start title = 'putSSOUser 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateAPISSOUserData = UpdateAPISSOUserData(groupIds: ["groupIds_example"], hasBlockedUsers: false, isProfileDMDisabled: false, isProfileCommentsPrivate: false, isProfileActivityPrivate: false, isCommentModeratorAdmin: false, isAdminAdmin: false, isAccountOwner: false, displayName: "displayName_example", displayLabel: "displayLabel_example", optedInSubscriptionNotifications: false, optedInNotifications: false, avatarSrc: "avatarSrc_example", loginCount: 123, createdFromUrlId: "createdFromUrlId_example", signUpDate: 123, email: "email_example", websiteUrl: "websiteUrl_example", username: "username_example", id: "id_example") // UpdateAPISSOUserData | 
let updateComments = true // Bool |  (選填)

DefaultAPI.putSSOUser(tenantId: tenantId, id: id, updateAPISSOUserData: updateAPISSOUserData, updateComments: updateComments) { (response, error) in
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