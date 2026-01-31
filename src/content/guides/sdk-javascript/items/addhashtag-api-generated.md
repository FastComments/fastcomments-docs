## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Response

Returns: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## Example

[inline-code-attrs-start title = 'addHashTag Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const createHashTagBody: CreateHashTagBody = {
  name: "bug",
  label: "Bug",
  description: "Reports issues that need developer attention in the production app",
  color: "#d9534f",
  requiresModeration: true
};
const result: AddHashTag200Response = await addHashTag(undefined, createHashTagBody);
[inline-code-end]
