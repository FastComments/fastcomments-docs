## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

返回: [`GetVotesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getVotes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchVotes(): Promise<void> {
  const tenantId: string = "acme-corp-01";
  const urlId: string = "article-2024-05-15";

  const response: GetVotesResponse1 = await getVotes(tenantId, urlId);

  // 範例說明如何存取回應中的可選欄位
  const firstVoteId: string | undefined = response?.votes?.[0]?.id;
}
[inline-code-end]