## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| search | string | 是 |  |
| locale | string | 否 |  |
| rating | string | 否 |  |
| page | number | 否 |  |

## 响应

返回：[`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## 示例

[inline-code-attrs-start title = 'getGifsSearch 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const search: string = "celebration fireworks";
  const locale: string = "fr-FR";
  const rating: string = "g";
  const page: number = 1;

  const fullResult: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
  const minimalResult: GetGifsSearchResponse = await getGifsSearch(tenantId, "dog memes");
}

runExample();
[inline-code-end]

---