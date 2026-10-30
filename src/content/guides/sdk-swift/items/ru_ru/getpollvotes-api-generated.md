The individual votes behind one poll's tallies, oldest first.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That keeps every query on the indexes the collection already has.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

## Parameters

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Да |  |
| voterId | string | query | Нет |  |
| optionId | string | query | Нет |  |
| skip | integer | query | Нет |  |

## Response

Возвращает: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (необязательно)
let optionId = "optionId_example" // String |  (необязательно)
let skip = 987 // Int |  (необязательно)

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