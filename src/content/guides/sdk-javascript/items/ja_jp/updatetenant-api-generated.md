## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateTenantBody | UpdateTenantBody | はい |  |

## Response

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'updateTenant の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdateTenant() {
  const tenantId: string = "tenant-abc123";
  const id: string = "config-456def";

  const updateTenantBody: UpdateTenantBody = {
    // 必須フィールド
    name: "Acme International",
    // オプションフィールドは省略可能、例: billingInfo, domainConfiguration
  };

  const response: APIEmptyResponse = await updateTenant(tenantId, id, updateTenantBody);
  console.log(response);
}
[inline-code-end]

---