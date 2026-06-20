---
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| yearNumber | number | query | Nee |  |
| monthNumber | number | query | Nee |  |
| dayNumber | number | query | Nee |  |
| skip | number | query | Nee |  |

## Antwoord

Retourneert: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsagesResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantDailyUsages Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen kunt u dit melden via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (optional)
let monthNumber = 987 // Double |  (optional)
let dayNumber = 987 // Double |  (optional)
let skip = 987 // Double |  (optional)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip) { (response, error) in
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