## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| meta | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenants Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-987";
  const basicResult: GetTenants200Response = await getTenants(tenantId);
  const meta: string = "include=domains,billing";
  const pagedResult: GetTenants200Response = await getTenants(tenantId, meta, 50);
  console.log(basicResult, pagedResult);
})();
[inline-code-end]
