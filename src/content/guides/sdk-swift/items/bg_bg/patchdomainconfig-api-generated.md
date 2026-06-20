---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| domainToUpdate | string | path | Да |  |

## Връща

Returns: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchDomainConfigResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за patchDomainConfig'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни фрагменти на код все още са бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let domainToUpdate = "domainToUpdate_example" // String | 
let patchDomainConfigParams = PatchDomainConfigParams(domain: "domain_example", emailFromName: "emailFromName_example", emailFromEmail: "emailFromEmail_example", logoSrc: "logoSrc_example", logoSrc100px: "logoSrc100px_example", footerUnsubscribeURL: "footerUnsubscribeURL_example", emailHeaders: "TODO") // PatchDomainConfigParams | 

DefaultAPI.patchDomainConfig(tenantId: tenantId, domainToUpdate: domainToUpdate, patchDomainConfigParams: patchDomainConfigParams) { (response, error) in
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