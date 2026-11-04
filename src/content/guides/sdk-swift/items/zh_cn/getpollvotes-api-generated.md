单个投票背后的各项投票记录，按时间从旧到新排列。

投票属于某条评论，因此投票记录总是按单个投票读取 - commentId 为必填项。这确保每个查询都能利用集合已有的索引。

遵循投票的隐私设置：匿名投票的投票记录无法读取（poll-anonymous），无论是通过此接口还是通过 id。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 是 |  |
| voterId | string | query | 否 |  |
| optionId | string | query | 否 |  |
| skip | integer | query | 否 |  |

## 响应

返回：[`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## 示例

[inline-code-attrs-start title = 'getPollVotes 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
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