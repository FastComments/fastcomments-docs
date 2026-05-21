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
const tenantId: string = 'tenant_9f3b21';
const createHashTagBody: CreateHashTagBody = { name: '#product-launch', description: 'Tags for the Q3 product launch campaign', isPublic: true };
const response: AddHashTag200Response = await addHashTag(tenantId, createHashTagBody);

const createHashTagBodyOnly: CreateHashTagBody = { name: '#user-feedback', description: 'Aggregated user feedback', isPublic: false };
const responseWithoutTenant: AddHashTag200Response = await addHashTag(undefined, createHashTagBodyOnly);
[inline-code-end]
