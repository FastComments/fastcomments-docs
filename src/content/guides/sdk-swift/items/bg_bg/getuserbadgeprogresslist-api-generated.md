## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Отговор

Връща: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgeProgressListResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadgeProgressList'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите кодови образци все още са в бета версия. За всеки проблем, моля, съобщете чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (по избор)
let limit = 987 // Double |  (по избор)
let skip = 987 // Double |  (по избор)

DefaultAPI.getUserBadgeProgressList(tenantId: tenantId, options: DefaultAPI.GetUserBadgeProgressListOptions(userId: userId, limit: limit, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]