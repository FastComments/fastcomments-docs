The individual votes behind one poll's tallies, oldest first.

一个投票的各个投票记录，按最早的顺序排列。

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

投票属于某条评论，因此投票始终一次读取一个投票——需要提供 commentId。这确保所有查询都使用集合已有的索引。

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

遵守投票的隐私设置：匿名投票的投票记录不可读取（poll-anonymous），无论是通过此接口还是通过 ID。

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

[inline-code-attrs-start title = 'getPollVotes 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 阶段。如有问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  （可选）
let optionId = "optionId_example" // String |  （可选）
let skip = 987 // Int |  （可选）

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