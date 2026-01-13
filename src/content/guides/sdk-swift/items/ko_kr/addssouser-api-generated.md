## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 응답

반환: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddSSOUserAPIResponse.swift)

## 예제

[inline-code-attrs-start title = 'addSSOUser 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createAPISSOUserData = CreateAPISSOUserData(groupIds: ["groupIds_example"], hasBlockedUsers: false, isProfileDMDisabled: false, isProfileCommentsPrivate: false, isProfileActivityPrivate: false, isCommentModeratorAdmin: false, isAdminAdmin: false, isAccountOwner: false, displayName: "displayName_example", displayLabel: "displayLabel_example", optedInSubscriptionNotifications: false, optedInNotifications: false, avatarSrc: "avatarSrc_example", loginCount: 123, createdFromUrlId: "createdFromUrlId_example", signUpDate: 123, email: "email_example", websiteUrl: "websiteUrl_example", username: "username_example", id: "id_example") // CreateAPISSOUserData | 

DefaultAPI.addSSOUser(tenantId: tenantId, createAPISSOUserData: createAPISSOUserData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]