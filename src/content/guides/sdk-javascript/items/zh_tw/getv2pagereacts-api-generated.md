## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

回傳: [`GetV2PageReacts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts200Response.ts)

## 範例

[inline-code-attrs-start title = 'getV2PageReacts 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_82f4b3a9";
const urlId: string = "https://news.site.com/articles/2026/06/15/product-launch";
const response: GetV2PageReacts200Response = await getV2PageReacts(tenantId, urlId);
console.log(response);
[inline-code-end]

---