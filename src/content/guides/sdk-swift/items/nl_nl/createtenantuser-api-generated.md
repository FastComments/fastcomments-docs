## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Respons

Retourneert: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTenantUser200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'createTenantUser Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen kunt u dit melden via http://github.com/OpenAPITools/openapi-generator/issues/new
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