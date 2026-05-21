## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_01';
  const id: string = 'user_4b7f9c';
  const result: GetTenantUser200Response = await getTenantUser(tenantId, id);
  const primaryEmail: string | undefined = result.user?.email;
  console.log(primaryEmail);
})();
[inline-code-end]
