投票に票を記録するか、既存の票を別の選択肢に移動します。投票者は1つの投票につき最大1票しか持てないため、同じ投票者に対して再度このAPIを呼び出すと、票が追加されるのではなく移動されます。

サイトの投票設定に従います。ログインユーザーのみ投票可能に設定されている場合、anonUserIdだけの投票は拒否され、匿名投票はIPごと・投票ごとにレート制限されます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## 例

[inline-code-attrs-start title = 'createPollVoteの例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createPollVoteBody = CreatePollVoteBody(commentId: "commentId_example", optionId: "optionId_example", userId: "userId_example", anonUserId: "anonUserId_example", ip: "ip_example") // CreatePollVoteBody | 

DefaultAPI.createPollVote(tenantId: tenantId, createPollVoteBody: createPollVoteBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]