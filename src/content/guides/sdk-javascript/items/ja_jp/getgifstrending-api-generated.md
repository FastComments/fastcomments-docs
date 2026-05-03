## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| locale | string | いいえ |  |
| rating | string | いいえ |  |
| page | number | いいえ |  |

## レスポンス

戻り値: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## 例

[inline-code-attrs-start title = 'getGifsTrending の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-01";
const trendingBasic: GifSearchResponse = await getGifsTrending(tenantId);

const locale: string = "en-GB";
const rating: string = "pg";
const page: number = 1;
const trendingWithOptions: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]