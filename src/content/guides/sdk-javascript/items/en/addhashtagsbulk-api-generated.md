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
const tenantId: string | undefined = undefined;
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [
    { name: 'marketing', color: '#FF6A00', externalId: 'mk-2026' },
    { name: 'customer-support', color: '#0057D9', externalId: 'cs-2026' }
  ]
};
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]
