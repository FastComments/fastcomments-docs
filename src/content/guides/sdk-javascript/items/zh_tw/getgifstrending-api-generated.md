## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| locale | string | 否 |  |
| rating | string | 否 |  |
| page | number | 否 |  |

## 回應

回傳: [`GetGifsTrending200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrending200Response.ts)

## 範例

[inline-code-attrs-start title = 'getGifsTrending 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = 'tenant_8b3f2c';
  const locale: string = 'en-US';
  const rating: string = 'pg';
  const page: number = 1;
  const result: GetGifsTrending200Response = await getGifsTrending(tenantId, locale, rating, page);
  console.log(result);
}
main();
[inline-code-end]