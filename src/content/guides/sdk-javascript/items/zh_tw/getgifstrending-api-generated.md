## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| locale | string | 否 |  |
| rating | string | 否 |  |
| page | number | 否 |  |

## 回應

回傳: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## 範例

[inline-code-attrs-start title = 'getGifsTrending 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const locale: string = 'en-US';
const rating: string = 'PG';
const page: number = 1;
const result: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---