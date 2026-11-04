投票をその場で編集し、集計結果を保持します。質問の変更、選択肢のラベル変更、投票の締め切りや再開、投票者の閲覧権限の変更などが可能です。選択肢は id で指定されます。選択肢の追加、削除、並べ替えを行うには、リスト全体を PUT してください。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |

## レスポンス

戻り値: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## 例

[inline-code-attrs-start title = 'patchPoll の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題が発生した場合は、http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
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