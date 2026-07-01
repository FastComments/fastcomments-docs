## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| commentId | string | Ja |  |
| broadcastId | string | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postUnFlagComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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