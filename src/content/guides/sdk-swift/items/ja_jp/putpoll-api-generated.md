コメントに投票(アンケート)を添付するか、既に添付されている投票の状態全体を設定します。

選択肢はidによって照合されます。既存の選択肢と同じidを持つ選択肢を送信すると、その投票数は保持され(新しいラベルと位置が適用され)、idなしで送信された選択肢は追加され、リストに含まれていない既存の選択肢は、それに投じられた投票とともに削除されます。

投票が存在する投票で既存の選択肢のidをすべて除外すると、それらはすべて削除されるため、その場合は replaceVotes=true が必要です。

## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| replaceVotes | boolean | query | いいえ |  |

## レスポンス

戻り値: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## 例

[inline-code-attrs-start title = 'putPoll の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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