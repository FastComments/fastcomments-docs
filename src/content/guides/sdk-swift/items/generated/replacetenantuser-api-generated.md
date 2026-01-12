## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | string | query | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Example

[inline-code-attrs-start title = 'replaceTenantUser Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let replaceTenantUserBody = ReplaceTenantUserBody(username: "username_example", email: "email_example", displayName: "displayName_example", websiteUrl: "websiteUrl_example", signUpDate: 123, locale: "locale_example", verified: false, loginCount: 123, optedInNotifications: false, optedInTenantNotifications: false, hideAccountCode: false, avatarSrc: "avatarSrc_example", isHelpRequestAdmin: false, isAccountOwner: false, isAdminAdmin: false, isBillingAdmin: false, isAnalyticsAdmin: false, isCustomizationAdmin: false, isManageDataAdmin: false, isCommentModeratorAdmin: false, isAPIAdmin: false, moderatorIds: ["moderatorIds_example"], digestEmailFrequency: 123, displayLabel: "displayLabel_example", createdFromUrlId: "createdFromUrlId_example", createdFromTenantId: "createdFromTenantId_example", lastLoginDate: 123, karma: 123) // ReplaceTenantUserBody | 
let updateComments = "updateComments_example" // String |  (optional)

DefaultAPI.replaceTenantUser(tenantId: tenantId, id: id, replaceTenantUserBody: replaceTenantUserBody, updateComments: updateComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
