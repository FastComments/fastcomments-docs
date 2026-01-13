## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| yearNumber | number | query | Ні |  |
| monthNumber | number | query | Ні |  |
| dayNumber | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsages200Response.swift)

## Приклад

[inline-code-attrs-start title = 'getTenantDailyUsages Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду усе ще в бета-версії. Якщо виникають проблеми, будь ласка, повідомте про це за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (необов'язково)
let monthNumber = 987 // Double |  (необов'язково)
let dayNumber = 987 // Double |  (необов'язково)
let skip = 987 // Double |  (необов'язково)

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