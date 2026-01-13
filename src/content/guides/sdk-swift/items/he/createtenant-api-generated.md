## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTenant200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של createTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. בכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createTenantBody = CreateTenantBody(name: "name_example", domainConfiguration: [APIDomainConfiguration(id: "id_example", domain: "domain_example", emailFromName: "emailFromName_example", emailFromEmail: "emailFromEmail_example", emailHeaders: "TODO", wpSyncToken: "wpSyncToken_example", wpSynced: false, wpURL: "wpURL_example", createdAt: Date(), autoAddedDate: Date(), siteType: ImportedSiteType(), logoSrc: "logoSrc_example", logoSrc100px: "logoSrc100px_example", footerUnsubscribeURL: "footerUnsubscribeURL_example", disableUnsubscribeLinks: false)], email: "email_example", signUpDate: 123, packageId: "packageId_example", paymentFrequency: 123, billingInfoValid: false, billingHandledExternally: false, createdBy: "createdBy_example", isSetup: false, billingInfo: BillingInfo(name: "name_example", address: "address_example", city: "city_example", state: "state_example", zip: "zip_example", country: "country_example", currency: "currency_example", email: "email_example"), stripeCustomerId: "stripeCustomerId_example", stripeSubscriptionId: "stripeSubscriptionId_example", stripePlanId: "stripePlanId_example", enableProfanityFilter: false, enableSpamFilter: false, removeUnverifiedComments: false, unverifiedCommentsTTLms: 123, commentsRequireApproval: false, autoApproveCommentOnVerification: false, sendProfaneToSpam: false, deAnonIpAddr: 123, meta: "TODO") // CreateTenantBody | 

DefaultAPI.createTenant(tenantId: tenantId, createTenantBody: createTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]