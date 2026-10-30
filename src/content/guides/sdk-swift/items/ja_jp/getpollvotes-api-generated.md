The individual votes behind one poll's tallies, oldest first.

1つの投票の集計の背後にある個々の投票、古い順に表示します。

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

投票はコメントに属するため、投票は常に1つの投票ごとに読み取られます - commentId が必須です。これにより、クエリはコレクションが既に持つインデックス上で実行されます。

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

投票のプライバシーを遵守します：匿名投票の投票は読み取れません（poll-anonymous）、ここでも ID でもです。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPollVotes 例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let commentId = "commentId_example" // String |
let voterId = "voterId_example" // String |  (オプション)
let optionId = "optionId_example" // String |  (オプション)
let skip = 987 // Int |  (オプション)

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

---