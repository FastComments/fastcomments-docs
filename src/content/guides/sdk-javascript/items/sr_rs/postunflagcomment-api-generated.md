## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'postUnFlagComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (optional)
    "tenant_abc",         // tenantId (optional)
    "sso_user_token_789"  // sso (optional)
  );
};
[inline-code-end]

---