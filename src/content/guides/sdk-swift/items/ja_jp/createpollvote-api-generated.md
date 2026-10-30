Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per
poll, so calling this again for the same voter moves their vote rather than adding one.

この操作はサイトの投票設定に従います。投票がログインユーザーのみに設定されている場合、anonUserId だけの投票は拒否され、匿名投票は投票ごとに IP 単位でレート制限されます。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

返却: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## 例

[inline-code-attrs-start title = 'createPollVote の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください。
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

---