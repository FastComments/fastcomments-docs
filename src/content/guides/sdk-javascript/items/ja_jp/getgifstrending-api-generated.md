## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|-------------|
| tenantId | string | はい |  |
| locale | string | いいえ |  |
| rating | string | いいえ |  |
| page | number | いいえ |  |

## レスポンス

戻り値: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## 例

[inline-code-attrs-start title = 'getGifsTrending の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media';
const locale: string = 'en-US';
const rating: string = 'pg-13';
const page: number = 2;
const trending: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---