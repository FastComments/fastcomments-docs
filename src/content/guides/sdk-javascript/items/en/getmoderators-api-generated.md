## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse.ts)

## Example

[inline-code-attrs-start title = 'getModerators Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_987654321";

const moderators: GetModeratorsResponse = await getModerators(tenantId);

const skip: number = 20;
const moreModerators: GetModeratorsResponse = await getModerators(tenantId, skip);
[inline-code-end]
