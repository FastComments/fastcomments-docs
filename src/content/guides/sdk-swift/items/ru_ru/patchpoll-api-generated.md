Edit a poll in place, keeping its tallies: change the question, relabel an option, close or reopen it, or change who may see the voters. Options are addressed by id - to add, remove or reorder them, PUT the full list.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример patchPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
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