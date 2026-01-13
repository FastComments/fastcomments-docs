## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTenantPackage200Response.swift)

## Example

[inline-code-attrs-start title = 'createTenantPackage Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still in beta. If you encounter any issues, please report them at http://github.com/OpenAPITools/openapi-generator/issues/new
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