## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'unBlockCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-84';
const commentId: string = 'cmt_987654321';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'erroneous moderation',
  unblockedBy: 'moderator.jane.doe@acme.com',
  notes: 'User appeal validated; content restored',
  restoreReplies: true
};
const sso: string | undefined = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const result: UnBlockCommentPublic200Response = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]
