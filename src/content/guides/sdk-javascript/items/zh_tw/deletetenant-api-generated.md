## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sure | string | No |  |

## 回應

Returns: [`DeleteTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantResponse.ts)

## 範例

[inline-code-attrs-start title = 'deleteTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "tenant_12345";
    const id: string = "resource_98765";
    const sure: string = "confirm";

    // 呼叫時包含可選的 'sure' 參數
    const responseWithSure: DeleteTenantResponse = await deleteTenant(tenantId, id, sure);
    console.log(responseWithSure);

    // 呼叫時不包含可選的 'sure' 參數
    const responseWithoutSure: DeleteTenantResponse = await deleteTenant(tenantId, id);
    console.log(responseWithoutSure);
}

runExample();
[inline-code-end]