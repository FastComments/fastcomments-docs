## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTenantUserResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'createTenantUser उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new के माध्यम से रिपोर्ट करें
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