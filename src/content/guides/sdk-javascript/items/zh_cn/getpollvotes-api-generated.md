单个投票背后的明细数据，按时间从旧到新排列。

投票属于某条评论，因此投票数据始终按单个投票读取 - commentId 为必填项。这确保每个查询都能命中集合已有的索引。

遵循投票的隐私设置：匿名投票的投票数据无法读取（poll-anonymous），无论是通过此接口还是通过 id。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| voterId | string | 否 |  |
| optionId | string | 否 |  |
| skip | number | 否 |  |

## 响应

返回：[`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getPollVotes 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]