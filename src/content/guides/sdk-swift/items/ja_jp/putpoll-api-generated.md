Attach a poll to a comment, or set the full state of the poll it already has.

Options are matched by id: an option sent with the id of an existing option keeps its votes (and takes the
new label and position), an option sent without an id is added, and existing options left out of the list
are removed along with the votes cast on them.

Keeping no existing option ids on a poll that has votes deletes all of them, so that needs replaceVotes=true.

## Parameters

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| replaceVotes | boolean | query | いいえ |  |

## レスポンス

返却: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## 例

[inline-code-attrs-start title = 'putPoll の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
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