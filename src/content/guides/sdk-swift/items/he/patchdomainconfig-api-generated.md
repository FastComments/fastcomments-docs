## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| domainToUpdate | string | path | כן |  |

## תגובה

מחזיר: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchDomainConfigResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-patchDomainConfig'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בבטא. עבור כל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
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