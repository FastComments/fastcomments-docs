## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| search | string | 是 |  |
| locale | string | 否 |  |
| rating | string | 否 |  |
| page | number | 否 |  |

## 回應

返回：[`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## 範例

[inline-code-attrs-start title = 'getGifsSearch 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_9f8b7c";
  const search: string = "funny cats";
  const locale: string = "en-US";
  const rating: string = "pg";
  const page: number = 1;

  const result: GetGifsSearchResponse = await getGifsSearch(
    tenantId,
    search,
    locale,
    rating,
    page
  );

  console.log(result);
}

demo();
[inline-code-end]

---