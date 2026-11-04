Прикріпіть опитування до коментаря або встановіть повний стан опитування, яке вже існує.

Варіанти зіставляються за id: варіант, надісланий з id наявного варіанта, зберігає свої голоси (і отримує нову
мітку та позицію), варіант без id додається, а наявні варіанти, відсутні у списку, видаляються разом із поданими за них голосами.

Якщо не залишити жодного наявного id варіанта в опитуванні, що має голоси, усі вони будуть видалені, тому потрібно replaceVotes=true.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| replaceVotes | boolean | query | Ні |  |

## Відповідь

Повертає: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. Про будь-яку проблему повідомляйте на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (необов'язково)

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