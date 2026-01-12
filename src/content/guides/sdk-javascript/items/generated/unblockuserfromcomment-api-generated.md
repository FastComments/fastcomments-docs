## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'unBlockUserFromComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7b3f2c9a";
const commentId: string = "comment_9d4a1f2b";
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: "User submitted successful appeal; block removed",
  moderatorId: "mod_jane_doe",
  effectiveAt: new Date().toISOString()
};
const userId: string = "user_5a8f3e21";
const anonUserId: string = "anon_2f7d9c88";
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(
  tenantId,
  commentId,
  unBlockFromCommentParams,
  userId,
  anonUserId
);
[inline-code-end]
