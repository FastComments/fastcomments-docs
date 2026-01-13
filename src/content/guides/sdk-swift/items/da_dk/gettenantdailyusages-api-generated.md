## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| yearNumber | number | query | Nej |  |
| monthNumber | number | query | Nej |  |
| dayNumber | number | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsages200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getTenantDailyUsages Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis der opstår problemer, bedes du rapportere via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (valgfri)
let monthNumber = 987 // Double |  (valgfri)
let dayNumber = 987 // Double |  (valgfri)
let skip = 987 // Double |  (valgfri)

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