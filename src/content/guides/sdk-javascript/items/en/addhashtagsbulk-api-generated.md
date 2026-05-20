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
const tenantId: string = 'tenant_8f3b2c';
const customConfig: CustomConfigParameters = { importance: 'low', notifyOnChange: false };
const tag: BulkCreateHashTagsBodyTagsInner = {
  name: 'feature-ui',
  description: 'Marks UI-related feature work',
  color: '#1E90FF',
  customConfig
};
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = { tags: [tag] };
const resultWithTenant: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
const resultWithoutTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, bulkCreateHashTagsBody);
[inline-code-end]
