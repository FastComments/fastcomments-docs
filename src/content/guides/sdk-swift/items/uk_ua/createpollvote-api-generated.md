---
Записати голос у опитуванні або перемістити існуючий голос до іншої опції. У виборця може бути не більше одного голосу на опитування, тому повторний виклик для того ж виборця переміщує його голос замість додавання нового.

Це дотримується налаштувань опитувань сайту: якщо голосування дозволено лише зареєстрованим користувачам, голос лише з anonUserId буде відхилено, а анонімні голоси обмежуються за кількістю запитів з одного IP на опитування.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Example

[inline-code-attrs-start title = 'createPollVote Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще у бета-версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createPollVoteBody = CreatePollVoteBody(commentId: "commentId_example", optionId: "optionId_example", userId: "userId_example", anonUserId: "anonUserId_example", ip: "ip_example") // CreatePollVoteBody | 

DefaultAPI.createPollVote(tenantId: tenantId, createPollVoteBody: createPollVoteBody) { (response, error) in
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