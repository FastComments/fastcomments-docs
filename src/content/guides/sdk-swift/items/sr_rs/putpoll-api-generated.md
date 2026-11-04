Прикачите анкету на коментар или поставите целокупно стање анкете коју већ има.

Опције се упарују по id: опција послата са id постојеће опције задржава своје гласове (и добија нову ознаку и позицију), опција послата без id се додаје, а постојеће опције које нису наведене на листи се уклањају заједно са гласовима датим на њима.

Ако се на анкети која има гласове не задржи ниједан постојећи id опције, сви гласови се бришу, тако да то захтева replaceVotes=true.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| replaceVotes | boolean | query | Не |  |

## Одговор

Враћа: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Пример

[inline-code-attrs-start title = 'putPoll пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (optional)

DefaultAPI.putPoll(tenantId: tenantId, commentId: commentId, commentPollPutInput: commentPollPutInput, replaceVotes: replaceVotes) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]