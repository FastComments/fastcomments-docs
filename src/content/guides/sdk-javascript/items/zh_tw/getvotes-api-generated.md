## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

回傳: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## 範例

[inline-code-attrs-start title = 'getVotes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-8f3b';
const refCampaign: string | undefined = 'newsletter-march2026'; // 可選的查詢參數
const urlId: string = `https://www.example.com/articles/2026/03/25/fastcomments-integration${refCampaign ? `?ref=${refCampaign}` : ''}`;

const votes: GetVotes200Response = await getVotes(tenantId, urlId);
[inline-code-end]