1つの投票の集計の背後にある個々の投票、古いものから順に表示されます。

投票はコメントに属するため、投票は常に1つの投票(ポール)ずつ読み取ります - commentId は必須です。これにより、すべてのクエリがコレクションが既に持っているインデックスを使用します。

投票のプライバシー設定に従います: 匿名投票の投票内容は読み取ることができません (poll-anonymous)。ここでも、id による取得でも同様です。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | query | はい |  |
| voterId | string | query | いいえ |  |
| optionId | string | query | いいえ |  |
| skip | integer | query | いいえ |  |

## レスポンス

戻り値: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## 例

[inline-code-attrs-start title = 'getPollVotes の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optional)
let optionId = "optionId_example" // String |  (optional)
let skip = 987 // Int |  (optional)

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