## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Yes |  |
| tenantId | string | No |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Response

Returns: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Example

[inline-code-attrs-start title = 'patchHashTag Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-2026";
const tenantId: string = "tenant_7b3f";
const updateHashTagBody: UpdateHashTagBody = {
  label: "Release 2026",
  description: "Tag used to group issues and features for the 2026 product launch",
  isActive: true
};
const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]
