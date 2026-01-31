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
  const tenantId: string = 'tenant-8432f9b1';
  const moderatorsFirstPage: GetModerators200Response = await getModerators(tenantId);
  const moderatorsSecondPage: GetModerators200Response = await getModerators(tenantId, 50);
  console.log(moderatorsFirstPage, moderatorsSecondPage);
})();
[inline-code-end]
