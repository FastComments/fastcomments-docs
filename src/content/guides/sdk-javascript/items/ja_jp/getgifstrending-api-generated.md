## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## レスポンス

戻り値: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## 例

[inline-code-attrs-start title = 'getGifsTrending の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const locale: string = "en-US";
  const rating: string = "PG-13";
  const page: number = 1;

  const trendingAll: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
  console.log(trendingAll);

  // 必要なパラメータのみを使用
  const trendingMinimal: GetGifsTrendingResponse = await getGifsTrending(tenantId);
  console.log(trendingMinimal);
}

runExample();
[inline-code-end]