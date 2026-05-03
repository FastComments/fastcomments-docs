## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| search | string | Yes |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## レスポンス

戻り値: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## 例

[inline-code-attrs-start title = 'getGifsSearch の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_fcm_42";
const search: string = "funny golden retriever";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 2;
const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---