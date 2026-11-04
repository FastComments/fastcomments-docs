在投票中記錄一票，或將現有的票移至不同的選項。每位投票者在每個投票中最多只有一票，因此對同一投票者再次呼叫此方法會移動其票，而不是新增一票。

此操作遵循網站的投票設定：若投票設定為僅限已登入使用者，僅帶有 anonUserId 的投票將被拒絕，且匿名投票會依 IP 並按每個投票進行速率限制。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createPollVoteBody | CreatePollVoteBody | 是 |  |

## 回應

回傳：[`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## 範例

[inline-code-attrs-start title = 'createPollVote 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId 為選填，此處予以省略
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]