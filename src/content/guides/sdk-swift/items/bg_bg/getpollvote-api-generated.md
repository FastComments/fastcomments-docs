## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`GetPollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoteResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са бета. При какъвто и да е проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getPollVote(tenantId: tenantId, id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]