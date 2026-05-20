## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantUsers Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "d1f3a8b0-7c2e-4f9a-9b8e-123456789abc";
  const responseWithoutSkip: GetTenantUsers200Response = await getTenantUsers(tenantId);
  const responseWithSkip: GetTenantUsers200Response = await getTenantUsers(tenantId, 20);
  console.log(responseWithoutSkip, responseWithSkip);
}
main();
[inline-code-end]
