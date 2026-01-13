## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| meta | string | 否 |  |
| skip | number | 否 |  |

## 回應

回傳: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenants 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9f2d1b7c';
  const meta: string = 'include=domains,billing,customConfig';
  const skip: number = 20;
  const response: GetTenants200Response = await getTenants(tenantId, meta, skip);
  console.log(response);
})();
[inline-code-end]

---