Редактирайте анкета на място, запазвайки резултатите: променете въпроса, пренаречете опция, затворете я или я отворете отново, или променете кой може да вижда гласувалите. Опциите се адресират по id – за добавяне, премахване или пренареждане, изпратете пълния списък с PUT.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |

## Отговор

Връща: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Пример

[inline-code-attrs-start title = 'patchPoll Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са бета. За какъвто и да е проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPatch = CommentPollPatch(question: "question_example", options: [CommentPollPatch_options_inner(label: "label_example", id: "id_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPatch | 

DefaultAPI.patchPoll(tenantId: tenantId, commentId: commentId, commentPollPatch: commentPollPatch) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]