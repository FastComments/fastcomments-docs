## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| search | string | 是 |  |
| locale | string | 否 |  |
| rating | string | 否 |  |
| page | number | 否 |  |

## 回應

回傳： [`GetGifsSearch200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearch200Response.ts)

## 範例

[inline-code-attrs-start title = 'getGifsSearch 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const search: string = "cat playing piano";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 1;
const result: GetGifsSearch200Response = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---