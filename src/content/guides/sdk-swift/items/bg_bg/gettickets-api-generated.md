## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Отговор

Връща: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getTickets Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващият кодов пример все още е в бета версия. За каквито и да е проблеми, моля, докладвайте ги на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (по избор)
let state = 987 // Double |  (по избор)
let skip = 987 // Double |  (по избор)
let limit = 987 // Double |  (по избор)

DefaultAPI.getTickets(tenantId: tenantId, options: DefaultAPI.GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]