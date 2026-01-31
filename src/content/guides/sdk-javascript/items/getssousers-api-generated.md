## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'getSSOUsers Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "fc_tenant_5872";
  const usersPage1: GetSSOUsers200Response = await getSSOUsers(tenantId);
  const skip: number = 50;
  const usersPage2: GetSSOUsers200Response = await getSSOUsers(tenantId, skip);
})();
[inline-code-end]
