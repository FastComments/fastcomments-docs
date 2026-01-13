## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## 예제

[inline-code-attrs-start title = 'updateTenant 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateTenantBody = UpdateTenantBody(name: "name_example", email: "email_example", signUpDate: 123, packageId: "packageId_example", paymentFrequency: 123, billingInfoValid: false, billingHandledExternally: false, createdBy: "createdBy_example", isSetup: false, domainConfiguration: [APIDomainConfiguration(id: "id_example", domain: "domain_example", emailFromName: "emailFromName_example", emailFromEmail: "emailFromEmail_example", emailHeaders: "TODO", wpSyncToken: "wpSyncToken_example", wpSynced: false, wpURL: "wpURL_example", createdAt: Date(), autoAddedDate: Date(), siteType: ImportedSiteType(), logoSrc: "logoSrc_example", logoSrc100px: "logoSrc100px_example", footerUnsubscribeURL: "footerUnsubscribeURL_example", disableUnsubscribeLinks: false)], billingInfo: BillingInfo(name: "name_example", address: "address_example", city: "city_example", state: "state_example", zip: "zip_example", country: "country_example", currency: "currency_example", email: "email_example"), stripeCustomerId: "stripeCustomerId_example", stripeSubscriptionId: "stripeSubscriptionId_example", stripePlanId: "stripePlanId_example", enableProfanityFilter: false, enableSpamFilter: false, removeUnverifiedComments: false, unverifiedCommentsTTLms: 123, commentsRequireApproval: false, autoApproveCommentOnVerification: false, sendProfaneToSpam: false, deAnonIpAddr: 123, meta: "TODO", managedByTenantId: "managedByTenantId_example") // UpdateTenantBody | 

DefaultAPI.updateTenant(tenantId: tenantId, id: id, updateTenantBody: updateTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]