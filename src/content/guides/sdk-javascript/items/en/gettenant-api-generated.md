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
(async () => {
  const tenantId: string = 'tenant-4f2b';
  const id: string = 'site-91a7';
  const tenantResponse: GetTenant200Response = await getTenant(tenantId, id);
  const billingInfo: BillingInfo | undefined = (tenantResponse as unknown as { billing?: BillingInfo }).billing;
})();
[inline-code-end]
