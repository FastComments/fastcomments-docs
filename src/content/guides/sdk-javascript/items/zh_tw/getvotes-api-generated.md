## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

返回：[`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getVotes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // 取得回應中可選屬性的範例
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]