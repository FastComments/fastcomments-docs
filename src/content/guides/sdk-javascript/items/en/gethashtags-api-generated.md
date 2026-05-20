## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | number | No |  |

## Response

Returns: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Example

[inline-code-attrs-start title = 'getHashTags Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3b9f1e2a-6c44-4f6a-9d2a-0b1234567890";
const page: number = 2;
const responseFirstPage: GetHashTags200Response = await getHashTags(tenantId);
const responseSecondPage: GetHashTags200Response = await getHashTags(tenantId, page);
[inline-code-end]
