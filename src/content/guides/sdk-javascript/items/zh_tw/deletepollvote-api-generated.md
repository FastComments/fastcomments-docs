---
撤回投票。投票所屬的選項會恢復其計數。

## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 回應

返回：[`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePollVoteResponse.ts)

## 範例

[inline-code-attrs-start title = 'deletePollVote 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePollVoteExample(): Promise<void> {
  const tenantId: string = "tenant_42abc";
  const pollId: string = "poll_7f9e2d";

  const result: DeletePollVoteResponse = await deletePollVote(tenantId, pollId);

  console.log(result);
}
[inline-code-end]

---