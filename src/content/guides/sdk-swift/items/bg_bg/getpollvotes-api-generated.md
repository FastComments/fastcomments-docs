Индивидуалните гласове зад сумирането на една анкета, най-старите първи.

Анкета принадлежи към коментар, затова гласовете винаги се четат по една анкета наведнъж – commentId е задължително. Това запазва всяка заявка върху индексите, които вече съществуват в колекцията.

Съобразява се с поверителността на анкетата: гласовете от анонимна анкета не могат да се четат (poll-anonymous), нито тук, нито по ID.

## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Да |  |
| voterId | string | query | Не |  |
| optionId | string | query | Не |  |
| skip | integer | query | Не |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример за getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са в бета. За всеки проблем, моля, докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
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

---