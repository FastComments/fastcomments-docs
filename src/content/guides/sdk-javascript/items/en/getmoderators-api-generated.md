## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerators200Response.ts)

## Example

[inline-code-attrs-start title = 'getModerators Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_acme-4f7a2b9c";
  const moderatorsPage1: GetModerators200Response = await getModerators(tenantId, 0);
  const moderatorsPage2: GetModerators200Response = await getModerators(tenantId, 50);
  console.log(moderatorsPage1, moderatorsPage2);
})();
[inline-code-end]
