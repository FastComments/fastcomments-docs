## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComment200Response.ts)

## Example

[inline-code-attrs-start title = 'getComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-01";
const commentId: string | undefined = "cmt_9f8e7d6c5b4a"; // optional-style variable
const result: GetComment200Response = await getComment(tenantId, commentId!);
[inline-code-end]
