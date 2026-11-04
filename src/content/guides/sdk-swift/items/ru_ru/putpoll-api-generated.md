Прикрепите опрос к комментарию или задайте полное состояние опроса, который уже есть.

Опции сопоставляются по id: опция, отправленная с id существующей опции, сохраняет свои голоса (и получает новую метку и позицию), опция без id добавляется, а существующие опции, отсутствующие в списке, удаляются вместе с отданными за них голосами.

Если не сохранить ни один из существующих id опций в опросе, за который уже есть голоса, все они будут удалены, поэтому требуется replaceVotes=true.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| replaceVotes | boolean | query | Нет |  |

## Ответ

Возвращает: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. О любых проблемах сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (необязательно)

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