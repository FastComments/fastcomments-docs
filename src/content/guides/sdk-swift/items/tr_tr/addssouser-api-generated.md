## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddSSOUserAPIResponse.swift)

## Örnek

[inline-code-attrs-start title = 'addSSOUser Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
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