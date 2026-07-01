## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| broadcastId | string | Nee |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'postUnFlagComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (optioneel)
    "tenant_abc",         // tenantId (optioneel)
    "sso_user_token_789"  // sso (optioneel)
  );
};
[inline-code-end]