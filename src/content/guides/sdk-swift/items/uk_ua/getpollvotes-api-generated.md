Окремі голоси за підсумками одного опитування, від найстаріших.

Опитування належить до коментаря, тому голоси завжди читаються по одному опитуванню за раз - commentId є обов'язковим. Це
зберігає кожен запит на індексах, які колекція вже має.

Дотримується приватності опитування: голоси анонімного опитування не можна прочитати (poll-anonymous), ні тут, ні за id.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | query | Так |  |
| voterId | string | query | Ні |  |
| optionId | string | query | Ні |  |
| skip | integer | query | Ні |  |

## Відповідь

Повертає: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще є бета-версією. Про будь-яку проблему, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optional)
let optionId = "optionId_example" // String |  (optional)
let skip = 987 // Int |  (optional)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]