## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| yearNumber | number | query | Не |  |
| monthNumber | number | query | Не |  |
| dayNumber | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsages200Response.swift)

## Пример

[inline-code-attrs-start title = 'getTenantDailyUsages Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери на код все още са бета. За всеки проблем, моля докладвайте на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (по избор)
let monthNumber = 987 // Double |  (по избор)
let dayNumber = 987 // Double |  (по избор)
let skip = 987 // Double |  (по избор)

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