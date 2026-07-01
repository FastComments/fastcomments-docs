## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | מחרוזת | שאילתא | כן |  |
| meta | מחרוזת | שאילתא | לא |  |
| skip | מספר | שאילתא | לא |  |

## Response

מחזיר: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTenants'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // מחרוזת | 
let meta = "meta_example" // מחרוזת | (אופציונלי)
let skip = 987 // Double | (אופציונלי)

DefaultAPI.getTenants(tenantId: tenantId, options: DefaultAPI.GetTenantsOptions(meta: meta, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]