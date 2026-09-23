## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## Example

[inline-code-attrs-start title = 'דוגמת getTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenant(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const id: string = "tenant_12345";

    const tenantResponse: GetTenantResponse = await getTenant(tenantId, id);

    // שדות אופציונליים בתגובה
    const billing: BillingInfo | undefined = tenantResponse.billingInfo;
    const domainConfig: APIDomainConfiguration | undefined = tenantResponse.domainConfiguration;
}
[inline-code-end]