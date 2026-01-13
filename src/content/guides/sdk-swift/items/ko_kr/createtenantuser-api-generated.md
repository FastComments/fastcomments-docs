## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTenantUser200Response.swift)

## 예제

[inline-code-attrs-start title = 'createTenantUser 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createTenantUserBody = CreateTenantUserBody(username: "username_example", email: "email_example", displayName: "displayName_example", websiteUrl: "websiteUrl_example", signUpDate: 123, locale: "locale_example", verified: false, loginCount: 123, optedInNotifications: false, optedInTenantNotifications: false, hideAccountCode: false, avatarSrc: "avatarSrc_example", isHelpRequestAdmin: false, isAccountOwner: false, isAdminAdmin: false, isBillingAdmin: false, isAnalyticsAdmin: false, isCustomizationAdmin: false, isManageDataAdmin: false, isCommentModeratorAdmin: false, isAPIAdmin: false, moderatorIds: ["moderatorIds_example"], digestEmailFrequency: 123, displayLabel: "displayLabel_example") // CreateTenantUserBody | 

DefaultAPI.createTenantUser(tenantId: tenantId, createTenantUserBody: createTenantUserBody) { (response, error) in
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