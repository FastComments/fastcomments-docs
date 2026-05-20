## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenant Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2c';
const id: string = 'org-84f7e2b1';
const optionalBillingInfo: BillingInfo | undefined = undefined;
const tenantResponse: GetTenant200Response = await getTenant(tenantId, id);
[inline-code-end]
