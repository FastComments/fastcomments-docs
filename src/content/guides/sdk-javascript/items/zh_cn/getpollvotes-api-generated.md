The individual votes behind one poll's tallies, oldest first.

一个投票的各个投票记录，按最早的顺序排列。

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

投票属于某条评论，因此投票始终一次读取一个投票——需要提供 commentId。这确保所有查询都使用集合已有的索引。

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

遵守投票的隐私设置：匿名投票的投票记录无法被读取（poll-anonymous），无论是在此处还是通过 ID。

## Parameters

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| voterId | string | 否 |  |
| optionId | string | 否 |  |
| skip | number | 否 |  |

## Response

返回：[`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

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

---