---
撤回投票。投票所投的选项会恢复其计数。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePollVoteResponse.ts)

## 示例

[inline-code-attrs-start title = 'deletePollVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePollVoteExample(): Promise<void> {
  const tenantId: string = "tenant_42abc";
  const pollId: string = "poll_7f9e2d";

  const result: DeletePollVoteResponse = await deletePollVote(tenantId, pollId);

  console.log(result);
}
[inline-code-end]

---