## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Svar

Returns: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'postUnFlagComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // kommentarId
    "brd_67890",          // broadcastId (valgfri)
    "tenant_abc",         // tenantId (valgfri)
    "sso_user_token_789"  // sso (valgfri)
  );
};
[inline-code-end]