## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| yearNumber | number | query | Ne |  |
| monthNumber | number | query | Ne |  |
| dayNumber | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odziv

Vrne: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsages200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer getTenantDailyUsages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še vedno beta. Za vsako težavo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (izbirno)
let monthNumber = 987 // Double |  (izbirno)
let dayNumber = 987 // Double |  (izbirno)
let skip = 987 // Double |  (izbirno)

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