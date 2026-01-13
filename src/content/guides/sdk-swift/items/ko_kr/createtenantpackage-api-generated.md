## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 응답

반환: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTenantPackage200Response.swift)

## 예제

[inline-code-attrs-start title = 'createTenantPackage 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createTenantPackageBody = CreateTenantPackageBody(name: "name_example", monthlyCostUSD: 123, yearlyCostUSD: 123, monthlyStripePlanId: "monthlyStripePlanId_example", yearlyStripePlanId: "yearlyStripePlanId_example", maxMonthlyPageLoads: 123, maxMonthlyAPICredits: 123, maxMonthlySmallWidgetsCredits: 123, maxMonthlyComments: 123, maxConcurrentUsers: 123, maxTenantUsers: 123, maxSSOUsers: 123, maxModerators: 123, maxDomains: 123, maxWhiteLabeledTenants: 123, maxMonthlyEventLogRequests: 123, hasWhiteLabeling: false, hasDebranding: false, hasLLMSpamDetection: false, forWhoText: "forWhoText_example", featureTaglines: ["featureTaglines_example"], hasAuditing: false, hasFlexPricing: false, enableSAML: false, flexPageLoadCostCents: 123, flexPageLoadUnit: 123, flexCommentCostCents: 123, flexCommentUnit: 123, flexSSOUserCostCents: 123, flexSSOUserUnit: 123, flexAPICreditCostCents: 123, flexAPICreditUnit: 123, flexSmallWidgetsCreditCostCents: 123, flexSmallWidgetsCreditUnit: 123, flexModeratorCostCents: 123, flexModeratorUnit: 123, flexAdminCostCents: 123, flexAdminUnit: 123, flexDomainCostCents: 123, flexDomainUnit: 123, flexChatGPTCostCents: 123, flexChatGPTUnit: 123, flexMinimumCostCents: 123, flexManagedTenantCostCents: 123, flexSSOAdminCostCents: 123, flexSSOAdminUnit: 123, flexSSOModeratorCostCents: 123, flexSSOModeratorUnit: 123) // CreateTenantPackageBody | 

DefaultAPI.createTenantPackage(tenantId: tenantId, createTenantPackageBody: createTenantPackageBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]