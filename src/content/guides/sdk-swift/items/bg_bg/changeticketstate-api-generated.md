## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ChangeTicketState200Response.swift)

## Пример

[inline-code-attrs-start title = 'changeTicketState Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери за код все още са в бета. За всеки проблем, моля, докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String | 
let id = "id_example" // String | 
let changeTicketStateBody = ChangeTicketStateBody(state: 123) // ChangeTicketStateBody | 

DefaultAPI.changeTicketState(tenantId: tenantId, userId: userId, id: id, changeTicketStateBody: changeTicketStateBody) { (response, error) in
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