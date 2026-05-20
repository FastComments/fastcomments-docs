## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Example

[inline-code-attrs-start title = 'getModerator Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-8421";
const id: string = "mod-7f3b2a9d";
const includeArchivedModerators: boolean | undefined = undefined; // optional parameter example
const result: GetModerator200Response = await getModerator(tenantId, id);
[inline-code-end]
