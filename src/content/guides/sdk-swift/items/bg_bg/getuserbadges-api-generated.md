## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| badgeId | string | query | Не |  |
| type | number | query | Не |  |
| displayedOnComments | boolean | query | Не |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са в бета. За всякакъв проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (по избор)
let badgeId = "badgeId_example" // String |  (по избор)
let type = 987 // Double |  (по избор)
let displayedOnComments = true // Bool |  (по избор)
let limit = 987 // Double |  (по избор)
let skip = 987 // Double |  (по избор)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
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