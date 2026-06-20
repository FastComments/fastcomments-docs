## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| domainToUpdate | string | path | Da |  |

## Odziv

Vrača: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PutDomainConfigResponse.swift)

## Primer

[inline-code-attrs-start title = 'putDomainConfig Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Če naletite na težavo, jo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let domainToUpdate = "domainToUpdate_example" // String | 
let updateDomainConfigParams = UpdateDomainConfigParams(domain: "domain_example", emailFromName: "emailFromName_example", emailFromEmail: "emailFromEmail_example", logoSrc: "logoSrc_example", logoSrc100px: "logoSrc100px_example", footerUnsubscribeURL: "footerUnsubscribeURL_example", emailHeaders: "TODO") // UpdateDomainConfigParams | 

DefaultAPI.putDomainConfig(tenantId: tenantId, domainToUpdate: domainToUpdate, updateDomainConfigParams: updateDomainConfigParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]