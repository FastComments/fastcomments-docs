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
const tag: string = "feature-request";
const tenantId: string = "acme_corp_001";
const updateHashTagBody: UpdateHashTagBody = {
  displayName: "Feature Request",
  description: "Requests for new product capabilities",
  color: "#007BFF",
  aliases: ["feat-request", "feature-idea"],
  isActive: true
};
const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]
