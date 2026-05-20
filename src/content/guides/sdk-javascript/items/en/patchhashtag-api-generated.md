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
(async (): Promise<void> => {
  const tag: string = "product-feedback";
  const tenantId: string = "acme-tenant-42";
  const updateHashTagBody: UpdateHashTagBody = {
    displayName: "Product Feedback",
    description: "Collects user feedback about products",
    isActive: true
  };
  const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
  console.log(result);
})();
[inline-code-end]
