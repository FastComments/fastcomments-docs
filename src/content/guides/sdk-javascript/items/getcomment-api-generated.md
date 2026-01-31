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
const tenantId: string = 'site-2468';
const id: string = 'cmt-5f1e3ab9d4';
const options: { includeReplies?: boolean } = { includeReplies: true };
const response: GetComment200Response = await getComment(tenantId, id);
const commentId: string | undefined = response?.data?.id;
[inline-code-end]
