## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| state | number | query | Не |  |
| skip | number | query | Не |  |
| limit | number | query | Не |  |

## Отговор

Връща: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTickets200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни кодове все още са в бета. При проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (незадължително)
let state = 987 // Double |  (незадължително)
let skip = 987 // Double |  (незадължително)
let limit = 987 // Double |  (незадължително)

DefaultAPI.getTickets(tenantId: tenantId, userId: userId, state: state, skip: skip, limit: limit) { (response, error) in
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