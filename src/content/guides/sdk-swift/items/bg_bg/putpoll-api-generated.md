Прикачване на анкета към коментар или задаване на пълното състояние на анкетата, която вече има.

Опциите се съпоставят по id: опция, изпратена с id на съществуваща опция, запазва своите гласове (и приема новия етикет и позиция), опция без id се добавя, а съществуващите опции, които липсват в списъка, се премахват заедно с подадените за тях гласове.

Ако не се запази нито едно съществуващо id на опция в анкета, която има гласове, всички те се изтриват, така че това изисква replaceVotes=true.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| replaceVotes | boolean | query | Не |  |

## Отговор

Връща: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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