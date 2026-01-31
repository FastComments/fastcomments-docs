## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentText200Response.ts)

## Example

[inline-code-attrs-start title = 'getCommentText Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_live_82f4c9b1';
const commentId: string = 'comment_4f3a2b1c';
const editKey: string = 'edk_9d8c7b6a5f4e3d2c1b0a';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1Njc4OSIsIm5hbWUiOiJKb2huIERvZSJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const commentTextWithAuth: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
const commentTextPublic: GetCommentText200Response = await getCommentText(tenantId, commentId);
[inline-code-end]
