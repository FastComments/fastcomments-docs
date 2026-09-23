在投票中记录一次投票，或将已有的投票移动到其他选项。每个投票者在同一投票中最多只能有一票，因此再次调用此接口时会移动该投票者的投票，而不是新增一票。

这遵循站点的投票设置：如果投票仅限登录用户，则仅使用 anonUserId 的投票会被拒绝，并且匿名投票会根据每个 IP 对每个投票进行速率限制。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'createPollVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId is optional and omitted here
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]

---