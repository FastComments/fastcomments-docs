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
const tenantId: string = 'acme-tenant-42';
const commentId: string = 'b3d1f9c2-7e8a-4d2e-9a1b-6c2e9f8a1234';
const commentResponse: GetComment200Response = await getComment(tenantId, commentId);
const apiComment: APIComment | undefined = (commentResponse as { comment?: APIComment }).comment;
const firstBadge: CommentUserBadgeInfo | undefined = apiComment?.user?.badges?.[0];
[inline-code-end]
