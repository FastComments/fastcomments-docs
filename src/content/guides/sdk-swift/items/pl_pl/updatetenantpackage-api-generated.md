## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład updateTenantPackage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateTenantPackageBody = UpdateTenantPackageBody(name: "name_example", monthlyCostUSD: 123, yearlyCostUSD: 123, maxMonthlyPageLoads: 123, maxMonthlyAPICredits: 123, maxMonthlyComments: 123, maxConcurrentUsers: 123, maxTenantUsers: 123, maxSSOUsers: 123, maxModerators: 123, maxDomains: 123, hasDebranding: false, hasWhiteLabeling: false, forWhoText: "forWhoText_example", featureTaglines: ["featureTaglines_example"], hasFlexPricing: false, flexPageLoadCostCents: 123, flexPageLoadUnit: 123, flexCommentCostCents: 123, flexCommentUnit: 123, flexSSOUserCostCents: 123, flexSSOUserUnit: 123, flexAPICreditCostCents: 123, flexAPICreditUnit: 123, flexModeratorCostCents: 123, flexModeratorUnit: 123, flexAdminCostCents: 123, flexAdminUnit: 123, flexDomainCostCents: 123, flexDomainUnit: 123, flexMinimumCostCents: 123) // UpdateTenantPackageBody | 

DefaultAPI.updateTenantPackage(tenantId: tenantId, id: id, updateTenantPackageBody: updateTenantPackageBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]