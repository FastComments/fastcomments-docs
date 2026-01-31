## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Response

Returns: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Example

[inline-code-attrs-start title = 'addHashTagsBulk Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8a7f2b39";
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [
    { name: "performance", description: "Tips to optimize rendering", color: "#00A86B", aliases: ["perf", "optimization"] },
    { name: "accessibility", description: "A11y best practices", color: "#FFB300" }
  ]
};
const responseWithTenant: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
const responseWithoutTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, {
  tags: [{ name: "security", description: "Security-related discussion", color: "#D7263D" }]
});
[inline-code-end]
